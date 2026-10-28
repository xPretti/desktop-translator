use serde::Serialize;

/// Erro serializável devolvido aos comandos Tauri: `{ code, message }`.
#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub code: String,
    pub message: String,
}

impl AppError {
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new("internal", message)
    }

    pub fn from_reqwest(error: &reqwest::Error) -> Self {
        if error.is_timeout() {
            Self::new("timeout", "A tradução demorou demais. Tente novamente.")
        } else {
            // Nunca inclui a URL/consulta do erro: ela pode conter texto privado.
            Self::new("network", "Falha de rede. Verifique sua conexão com a internet.")
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}
