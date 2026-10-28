use std::fs;
use std::path::{Path, PathBuf};

use crate::models::error::AppError;
use crate::models::settings::Settings;

/// Grava em arquivo temporário e renomeia, evitando arquivos corrompidos em caso de falha.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), AppError> {
    let io = |e: std::io::Error| AppError::internal(format!("Falha ao gravar arquivo: {e}"));
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io)?;
    }
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, bytes).map_err(io)?;
    fs::rename(&tmp, path).map_err(io)
}

pub struct SettingsStore {
    path: PathBuf,
}

impl SettingsStore {
    pub fn new(dir: &Path) -> Self {
        Self { path: dir.join("settings.json") }
    }

    /// Arquivo ausente ou inválido resulta nos valores padrão.
    pub fn load(&self) -> Settings {
        fs::read_to_string(&self.path)
            .ok()
            .and_then(|raw| serde_json::from_str::<Settings>(&raw).ok())
            .map(Settings::sanitized)
            .unwrap_or_default()
    }

    pub fn save(&self, settings: &Settings) -> Result<(), AppError> {
        let bytes = serde_json::to_vec_pretty(settings)
            .map_err(|e| AppError::internal(format!("Falha ao serializar configurações: {e}")))?;
        write_atomic(&self.path, &bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("dt-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn save_then_load_roundtrip() {
        let dir = temp_dir("roundtrip");
        let store = SettingsStore::new(&dir);
        let mut settings = Settings::default();
        settings.translation.target_languages = vec!["es".into(), "fr".into()];
        store.save(&settings).unwrap();
        assert_eq!(store.load().translation.target_languages, vec!["es".to_string(), "fr".to_string()]);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_or_corrupt_file_yields_defaults() {
        let dir = temp_dir("corrupt");
        let store = SettingsStore::new(&dir);
        assert_eq!(store.load(), Settings::default());
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("settings.json"), "{ not json").unwrap();
        assert_eq!(store.load(), Settings::default());
        let _ = fs::remove_dir_all(&dir);
    }
}
