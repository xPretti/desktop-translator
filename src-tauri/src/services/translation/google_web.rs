//! Provedor NÃO OFICIAL: endpoint público usado pelo site/extensões do Google Tradutor.
//! Não é uma API documentada nem tem garantia de estabilidade, cota ou continuidade:
//! pode mudar ou bloquear requisições a qualquer momento.

use serde_json::Value;

use super::provider::{error_for_status, unexpected_response, BoxFuture, ProviderRequest, ProviderResponse, TranslationProvider};
use crate::models::error::AppError;

const ENDPOINT: &str = "https://translate.googleapis.com/translate_a/single";

pub struct GoogleWebProvider {
    client: reqwest::Client,
}

impl GoogleWebProvider {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

/// Resposta: `[[["traduzido","original",...], ...], null, "idioma-detectado", ...]`.
pub fn parse_response(value: &Value) -> Result<ProviderResponse, AppError> {
    let segments = value.get(0).and_then(Value::as_array).ok_or_else(unexpected_response)?;
    let translated: String = segments
        .iter()
        .filter_map(|segment| segment.get(0).and_then(Value::as_str))
        .collect();
    if translated.is_empty() {
        return Err(unexpected_response());
    }
    let detected = value.get(2).and_then(Value::as_str).map(str::to_string);
    Ok(ProviderResponse { translated, detected })
}

impl TranslationProvider for GoogleWebProvider {
    fn translate<'a>(&'a self, request: &'a ProviderRequest) -> BoxFuture<'a, Result<ProviderResponse, AppError>> {
        Box::pin(async move {
            // O texto vai no corpo (POST), não na URL, para não aparecer em logs de proxies.
            let response = self
                .client
                .post(ENDPOINT)
                .query(&[
                    ("client", "gtx"),
                    ("sl", request.source.as_str()),
                    ("tl", request.target.as_str()),
                    ("dt", "t"),
                ])
                .form(&[("q", request.text.as_str())])
                .timeout(request.timeout)
                .send()
                .await
                .map_err(|e| AppError::from_reqwest(&e))?;
            if !response.status().is_success() {
                return Err(error_for_status(response.status()));
            }
            let value: Value = response.json().await.map_err(|_| unexpected_response())?;
            parse_response(&value)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn joins_segments_and_reads_detected_language() {
        let value = json!([[["Olá. ", "Hello. ", null, null, 10], ["Tudo bem?", "How are you?", null, null, 10]], null, "en"]);
        let parsed = parse_response(&value).unwrap();
        assert_eq!(parsed.translated, "Olá. Tudo bem?");
        assert_eq!(parsed.detected.as_deref(), Some("en"));
    }

    #[test]
    fn rejects_unexpected_shapes() {
        assert!(parse_response(&json!({"error": "x"})).is_err());
        assert!(parse_response(&json!([[]])).is_err());
    }
}
