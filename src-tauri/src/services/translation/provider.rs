use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use reqwest::StatusCode;

use crate::models::error::AppError;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub struct ProviderRequest {
    pub text: String,
    pub source: String,
    pub target: String,
    pub timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct ProviderResponse {
    pub translated: String,
    pub detected: Option<String>,
}

/// Contrato de um provedor de tradução (substituível).
pub trait TranslationProvider: Send + Sync {
    fn translate<'a>(&'a self, request: &'a ProviderRequest) -> BoxFuture<'a, Result<ProviderResponse, AppError>>;
}

pub fn error_for_status(status: StatusCode) -> AppError {
    match status.as_u16() {
        400 => AppError::new("invalid_request", "O provedor recusou a requisição (texto ou idioma inválido)."),
        401 | 403 => AppError::new("auth", "Chave de API inválida ou sem permissão."),
        429 => AppError::new("quota", "Limite de uso do provedor atingido. Tente novamente mais tarde."),
        _ => AppError::new("unavailable", "O serviço de tradução está indisponível no momento."),
    }
}

pub fn unexpected_response() -> AppError {
    AppError::new("unavailable", "O serviço de tradução retornou uma resposta inesperada.")
}
