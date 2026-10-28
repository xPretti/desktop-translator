use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationRequest {
    pub text: String,
    pub source_language: String,
    /// Destinos em ordem de prioridade; o primeiro compatível com o idioma detectado é usado.
    pub target_languages: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationResult {
    pub original_text: String,
    pub translated_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<String>,
    pub target_language: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatus {
    pub translation_enabled: bool,
    pub capture_enabled: bool,
    pub capture_supported: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PopupPayload {
    pub text: String,
    pub truncated: bool,
    pub request_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
