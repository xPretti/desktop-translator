use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Light,
    Dark,
    #[default]
    Auto,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlacementSide {
    #[default]
    Below,
    Above,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlacementFallback {
    #[default]
    Flip,
    Clamp,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AfterTranslation {
    #[default]
    None,
    Copy,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NewSelectionBehavior {
    #[default]
    Replace,
    Close,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CloseAction {
    #[default]
    Tray,
    Quit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Appearance {
    pub theme: ThemeMode,
    pub max_width: i32,
    pub max_height: i32,
    pub min_width: i32,
    pub border_color: String,
    pub border_width: i32,
    pub background_color: String,
    pub text_color: String,
    pub font_size: i32,
    pub font_family: String,
    pub padding: i32,
    pub border_radius: i32,
    pub opacity: f64,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Auto,
            max_width: 420,
            max_height: 320,
            min_width: 240,
            border_color: String::new(),
            border_width: 1,
            background_color: String::new(),
            text_color: String::new(),
            font_size: 14,
            font_family: String::new(),
            padding: 14,
            border_radius: 10,
            opacity: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Positioning {
    pub popup_side: PlacementSide,
    pub button_side: PlacementSide,
    pub offset_x: i32,
    pub offset_y: i32,
    pub min_distance: i32,
    pub button_distance: i32,
    pub screen_margin: i32,
    pub fallback: PlacementFallback,
}

impl Default for Positioning {
    fn default() -> Self {
        Self {
            popup_side: PlacementSide::Below,
            button_side: PlacementSide::Below,
            offset_x: 0,
            offset_y: 0,
            min_distance: 8,
            button_distance: 6,
            screen_margin: 12,
            fallback: PlacementFallback::Flip,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TranslationSettings {
    pub source_language: String,
    /// Idiomas de destino em ordem de prioridade (1 a 5). O primeiro é o principal; se o texto já
    /// estiver nele, usa-se o próximo da lista.
    #[serde(default = "no_languages")]
    pub target_languages: Vec<String>,
    /// Campo antigo (`targetLanguage`), lido apenas para migrar configurações existentes.
    #[serde(default, rename = "targetLanguage", skip_serializing)]
    pub legacy_target_language: Option<String>,
    pub auto_translate: bool,
    pub max_text_length: i32,
    pub timeout_seconds: i32,
    pub cache_enabled: bool,
    pub clipboard_fallback: bool,
    pub after_translation: AfterTranslation,
    pub translation_enabled: bool,
    pub capture_enabled: bool,
}

fn no_languages() -> Vec<String> {
    Vec::new()
}

pub fn default_target_languages() -> Vec<String> {
    vec!["pt".to_string(), "en".to_string()]
}

pub const MAX_TARGET_LANGUAGES: usize = 5;

impl Default for TranslationSettings {
    fn default() -> Self {
        Self {
            source_language: "auto".into(),
            target_languages: default_target_languages(),
            legacy_target_language: None,
            auto_translate: false,
            max_text_length: 3000,
            timeout_seconds: 10,
            cache_enabled: true,
            clipboard_fallback: true,
            after_translation: AfterTranslation::None,
            translation_enabled: true,
            capture_enabled: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Shortcuts {
    pub toggle_capture: String,
    pub open_main: String,
    pub translate_selection: String,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            toggle_capture: "Ctrl+Alt+Shift+T".into(),
            open_main: "Ctrl+Alt+M".into(),
            translate_selection: "Ctrl+Alt+T".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Behavior {
    pub launch_at_startup: bool,
    pub start_minimized: bool,
    pub minimize_to_tray: bool,
    pub close_action: CloseAction,
    pub popup_auto_close: bool,
    pub popup_auto_close_seconds: i32,
    pub button_auto_hide_seconds: i32,
    pub close_on_outside_click: bool,
    pub new_selection_behavior: NewSelectionBehavior,
    pub shortcuts: Shortcuts,
}

impl Default for Behavior {
    fn default() -> Self {
        Self {
            launch_at_startup: false,
            start_minimized: false,
            minimize_to_tray: true,
            close_action: CloseAction::Tray,
            popup_auto_close: false,
            popup_auto_close_seconds: 15,
            button_auto_hide_seconds: 5,
            close_on_outside_click: true,
            new_selection_behavior: NewSelectionBehavior::Replace,
            shortcuts: Shortcuts::default(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub appearance: Appearance,
    pub positioning: Positioning,
    pub translation: TranslationSettings,
    pub behavior: Behavior,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveResult {
    pub settings: Settings,
    pub warnings: Vec<String>,
}

fn clamp_i(value: i32, min: i32, max: i32) -> i32 {
    value.clamp(min, max)
}

fn clean_color(value: &str) -> String {
    let v = value.trim();
    let hex = v.strip_prefix('#').unwrap_or("");
    let valid = v.starts_with('#')
        && matches!(hex.len(), 3 | 6 | 8)
        && hex.chars().all(|c| c.is_ascii_hexdigit());
    if valid {
        v.to_string()
    } else {
        String::new()
    }
}

fn clean_font(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(c, ' ' | ',' | '-' | '_' | '"' | '\''))
        .take(120)
        .collect::<String>()
        .trim()
        .to_string()
}

/// Código de idioma válido (ex.: "pt", "zh-CN") ou `None`.
pub fn valid_language(value: &str) -> Option<String> {
    let v = value.trim();
    let ok = (2..=8).contains(&v.len()) && v.chars().all(|c| c.is_ascii_alphanumeric() || c == '-');
    ok.then(|| v.to_string())
}

fn clean_source_language(value: &str) -> String {
    let v = value.trim();
    if v == "auto" {
        return v.to_string();
    }
    valid_language(v).unwrap_or_else(|| "auto".to_string())
}

/// Normaliza a lista de destinos: códigos válidos, sem duplicatas, no máximo 5; migra o campo antigo.
fn clean_target_languages(list: Vec<String>, legacy: Option<String>) -> Vec<String> {
    let migrating = list.is_empty() && legacy.is_some();
    let mut source = list;
    if migrating {
        source.extend(legacy);
    }
    let mut cleaned: Vec<String> = Vec::new();
    for item in source {
        if let Some(code) = valid_language(&item) {
            if !cleaned.iter().any(|c| c.eq_ignore_ascii_case(&code)) {
                cleaned.push(code);
            }
        }
        if cleaned.len() == MAX_TARGET_LANGUAGES {
            break;
        }
    }
    if cleaned.is_empty() {
        return default_target_languages();
    }
    if migrating && cleaned.len() == 1 {
        let fallback = if cleaned[0].eq_ignore_ascii_case("en") { "pt" } else { "en" };
        cleaned.push(fallback.to_string());
    }
    cleaned
}

fn clean_shortcut(value: &str) -> String {
    value
        .trim()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '+')
        .take(40)
        .collect()
}

impl Settings {
    /// Normaliza qualquer valor recebido (arquivo ou frontend) para limites seguros.
    pub fn sanitized(mut self) -> Self {
        let a = &mut self.appearance;
        a.min_width = clamp_i(a.min_width, 120, 600);
        a.max_width = clamp_i(a.max_width, 200, 900).max(a.min_width);
        a.max_height = clamp_i(a.max_height, 100, 900);
        a.border_width = clamp_i(a.border_width, 0, 8);
        a.font_size = clamp_i(a.font_size, 10, 32);
        a.padding = clamp_i(a.padding, 0, 40);
        a.border_radius = clamp_i(a.border_radius, 0, 32);
        a.opacity = if a.opacity.is_finite() { a.opacity.clamp(0.4, 1.0) } else { 1.0 };
        a.border_color = clean_color(&a.border_color);
        a.background_color = clean_color(&a.background_color);
        a.text_color = clean_color(&a.text_color);
        a.font_family = clean_font(&a.font_family);

        let p = &mut self.positioning;
        p.offset_x = clamp_i(p.offset_x, -200, 200);
        p.offset_y = clamp_i(p.offset_y, -200, 200);
        p.min_distance = clamp_i(p.min_distance, 0, 100);
        p.button_distance = clamp_i(p.button_distance, 0, 100);
        p.screen_margin = clamp_i(p.screen_margin, 0, 100);

        let t = &mut self.translation;
        t.source_language = clean_source_language(&t.source_language);
        let targets = std::mem::take(&mut t.target_languages);
        t.target_languages = clean_target_languages(targets, t.legacy_target_language.take());
        t.max_text_length = clamp_i(t.max_text_length, 1, 5000);
        t.timeout_seconds = clamp_i(t.timeout_seconds, 2, 60);

        let b = &mut self.behavior;
        b.popup_auto_close_seconds = clamp_i(b.popup_auto_close_seconds, 2, 300);
        b.button_auto_hide_seconds = clamp_i(b.button_auto_hide_seconds, 0, 60);
        b.shortcuts.toggle_capture = clean_shortcut(&b.shortcuts.toggle_capture);
        b.shortcuts.open_main = clean_shortcut(&b.shortcuts.open_main);
        b.shortcuts.translate_selection = clean_shortcut(&b.shortcuts.translate_selection);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_clamps_out_of_range_values() {
        let mut s = Settings::default();
        s.appearance.max_width = 99_999;
        s.appearance.font_size = 1;
        s.appearance.opacity = f64::NAN;
        s.translation.max_text_length = 0;
        s.translation.timeout_seconds = 999;
        let s = s.sanitized();
        assert_eq!(s.appearance.max_width, 900);
        assert_eq!(s.appearance.font_size, 10);
        assert_eq!(s.appearance.opacity, 1.0);
        assert_eq!(s.translation.max_text_length, 1);
        assert_eq!(s.translation.timeout_seconds, 60);
    }

    #[test]
    fn sanitize_rejects_invalid_colors_and_languages() {
        let mut s = Settings::default();
        s.appearance.background_color = "red; background:url(x)".into();
        s.appearance.text_color = "#1A2b3C".into();
        s.translation.target_languages = vec!["../etc".into(), "en".into(), "EN".into()];
        let s = s.sanitized();
        assert_eq!(s.appearance.background_color, "");
        assert_eq!(s.appearance.text_color, "#1A2b3C");
        assert_eq!(s.translation.target_languages, vec!["en".to_string()]);
    }

    #[test]
    fn max_width_never_below_min_width() {
        let mut s = Settings::default();
        s.appearance.min_width = 500;
        s.appearance.max_width = 300;
        let s = s.sanitized();
        assert!(s.appearance.max_width >= s.appearance.min_width);
    }

    #[test]
    fn missing_fields_fall_back_to_defaults() {
        let s: Settings = serde_json::from_str(r#"{"translation":{"timeoutSeconds":5}}"#).unwrap();
        assert_eq!(s.translation.timeout_seconds, 5);
        assert_eq!(s.appearance, Appearance::default());
    }

    #[test]
    fn migrates_legacy_single_target_language() {
        let s: Settings = serde_json::from_str(r#"{"translation":{"targetLanguage":"es"}}"#).unwrap();
        assert_eq!(s.sanitized().translation.target_languages, vec!["es".to_string(), "en".to_string()]);
    }

    #[test]
    fn limits_and_dedupes_target_languages() {
        let mut s = Settings::default();
        s.translation.target_languages = ["pt", "en", "es", "fr", "de", "it", "ja"].iter().map(|c| c.to_string()).collect();
        assert_eq!(s.sanitized().translation.target_languages.len(), MAX_TARGET_LANGUAGES);
        let mut empty = Settings::default();
        empty.translation.target_languages = vec![];
        assert_eq!(empty.sanitized().translation.target_languages, default_target_languages());
    }
}
