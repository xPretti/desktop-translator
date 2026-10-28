//! Serviço de tradução: validação, cache, fallback entre idiomas de destino e provedor.
//!
//! Provedor atual: Google Tradutor (endpoint não oficial, ver `google_web`). Para adicionar outro,
//! implemente `TranslationProvider` e use-o em `translate`.

mod cache;
pub mod google_web;
pub mod provider;

use std::sync::Mutex;
use std::time::Duration;

use crate::models::error::AppError;
use crate::models::settings::{valid_language, TranslationSettings, MAX_TARGET_LANGUAGES};
use crate::models::translation::{TranslationRequest, TranslationResult};

use cache::TranslationCache;
use provider::{ProviderRequest, ProviderResponse, TranslationProvider};

pub struct TranslationService {
    client: reqwest::Client,
    cache: Mutex<TranslationCache>,
}

fn normalize(code: &str) -> String {
    let lower = code.trim().to_ascii_lowercase();
    match lower.as_str() {
        "iw" => "he".to_string(),
        "jw" => "jv".to_string(),
        _ => lower,
    }
}

/// Dois códigos representam o mesmo idioma? "pt" == "pt-BR"; "zh-CN" != "zh-TW".
pub fn same_language(a: &str, b: &str) -> bool {
    let (a, b) = (normalize(a), normalize(b));
    if a == b {
        return true;
    }
    let (base_a, region_a) = a.split_once('-').map_or((a.as_str(), None), |(x, y)| (x, Some(y)));
    let (base_b, region_b) = b.split_once('-').map_or((b.as_str(), None), |(x, y)| (x, Some(y)));
    base_a == base_b && (region_a.is_none() || region_b.is_none())
}

/// Traduz para o primeiro destino cujo idioma difere do idioma do texto.
/// - Origem automática: usa o idioma detectado pelo provedor (o texto já está no destino → próximo).
/// - Origem explícita: destinos iguais à origem são ignorados.
/// Se o texto já estiver em todos os destinos, devolve o resultado do primeiro.
pub async fn translate_with_fallback(
    provider: &dyn TranslationProvider,
    text: &str,
    source: &str,
    targets: &[String],
    timeout: Duration,
) -> Result<(ProviderResponse, String), AppError> {
    let mut first: Option<(ProviderResponse, String)> = None;
    for target in targets {
        if source != "auto" && same_language(source, target) {
            continue;
        }
        let response = provider
            .translate(&ProviderRequest {
                text: text.to_string(),
                source: source.to_string(),
                target: target.clone(),
                timeout,
            })
            .await?;
        let already_in_target = response.detected.as_deref().is_some_and(|d| same_language(d, target));
        if !already_in_target {
            return Ok((response, target.clone()));
        }
        if first.is_none() {
            first = Some((response, target.clone()));
        }
    }
    first.ok_or_else(|| AppError::new("invalid_request", "O idioma de origem é igual ao(s) idioma(s) de destino."))
}

impl TranslationService {
    pub fn new() -> Self {
        let client = reqwest::Client::builder().user_agent("DesktopTranslator/0.1").build().unwrap_or_default();
        Self { client, cache: Mutex::new(TranslationCache::new(200)) }
    }

    pub async fn translate(
        &self,
        request: TranslationRequest,
        settings: &TranslationSettings,
    ) -> Result<TranslationResult, AppError> {
        let text = request.text.trim().to_string();
        if text.is_empty() {
            return Err(AppError::new("empty_text", "Nenhum texto para traduzir."));
        }
        if text.chars().count() > settings.max_text_length as usize {
            return Err(AppError::new(
                "text_too_long",
                format!("O texto excede o limite de {} caracteres.", settings.max_text_length),
            ));
        }
        let source = request.source_language.trim().to_string();
        if source != "auto" && valid_language(&source).is_none() {
            return Err(AppError::new("invalid_request", "Idioma de origem inválido."));
        }
        let mut targets: Vec<String> = Vec::new();
        for target in &request.target_languages {
            let code = valid_language(target).ok_or_else(|| AppError::new("invalid_request", "Idioma de destino inválido."))?;
            if !targets.iter().any(|t| same_language(t, &code) && t.eq_ignore_ascii_case(&code)) {
                targets.push(code);
            }
        }
        targets.truncate(MAX_TARGET_LANGUAGES);
        if targets.is_empty() {
            return Err(AppError::new("invalid_request", "Informe ao menos um idioma de destino."));
        }

        let key = format!("{source}|{}|{text}", targets.join(","));
        if settings.cache_enabled {
            if let Some(hit) = self.cache.lock().unwrap_or_else(|e| e.into_inner()).get(&key) {
                return Ok(hit);
            }
        }

        let provider = google_web::GoogleWebProvider::new(self.client.clone());
        let timeout = Duration::from_secs(settings.timeout_seconds as u64);
        let (response, target) = translate_with_fallback(&provider, &text, &source, &targets, timeout).await?;

        let result = TranslationResult {
            original_text: text,
            translated_text: response.translated,
            detected_language: response.detected,
            target_language: target,
        };
        if settings.cache_enabled {
            self.cache.lock().unwrap_or_else(|e| e.into_inner()).put(key, result.clone());
        }
        Ok(result)
    }
}

impl Default for TranslationService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use provider::BoxFuture;

    /// Provedor simulado: devolve o idioma detectado fixo e registra os destinos pedidos.
    struct Mock {
        detected: &'static str,
        calls: Mutex<Vec<String>>,
    }

    impl TranslationProvider for Mock {
        fn translate<'a>(&'a self, request: &'a ProviderRequest) -> BoxFuture<'a, Result<ProviderResponse, AppError>> {
            Box::pin(async move {
                self.calls.lock().unwrap().push(request.target.clone());
                Ok(ProviderResponse {
                    translated: format!("[{}] {}", request.target, request.text),
                    detected: Some(self.detected.to_string()),
                })
            })
        }
    }

    fn run(detected: &'static str, source: &str, targets: &[&str]) -> (String, Vec<String>) {
        let mock = Mock { detected, calls: Mutex::new(Vec::new()) };
        let targets: Vec<String> = targets.iter().map(|t| t.to_string()).collect();
        let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
        let (_, target) = runtime
            .block_on(translate_with_fallback(&mock, "texto", source, &targets, Duration::from_secs(1)))
            .unwrap();
        let calls = mock.calls.lock().unwrap().clone();
        (target, calls)
    }

    #[test]
    fn uses_primary_target_when_text_is_in_another_language() {
        assert_eq!(run("en", "auto", &["pt", "en"]), ("pt".to_string(), vec!["pt".to_string()]));
    }

    #[test]
    fn falls_back_to_next_target_when_text_is_already_in_primary() {
        assert_eq!(run("pt", "auto", &["pt", "en"]), ("en".to_string(), vec!["pt".to_string(), "en".to_string()]));
    }

    #[test]
    fn regional_variants_count_as_the_same_language() {
        assert_eq!(run("pt-BR", "auto", &["pt", "es"]).0, "es");
    }

    #[test]
    fn explicit_source_skips_equal_targets_without_calling_the_provider() {
        assert_eq!(run("en", "pt", &["pt", "en"]), ("en".to_string(), vec!["en".to_string()]));
    }

    #[test]
    fn returns_first_result_when_text_is_in_every_target() {
        assert_eq!(run("pt", "auto", &["pt"]).0, "pt");
    }

    #[test]
    fn compares_language_codes() {
        assert!(same_language("pt", "pt-BR"));
        assert!(same_language("iw", "he"));
        assert!(!same_language("zh-CN", "zh-TW"));
        assert!(!same_language("en", "es"));
    }
}
