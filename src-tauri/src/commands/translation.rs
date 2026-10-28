use tauri::State;

use crate::models::error::AppError;
use crate::models::translation::{ServiceStatus, TranslationRequest, TranslationResult};
use crate::platform;
use crate::state::AppState;

#[tauri::command]
pub async fn translate(state: State<'_, AppState>, request: TranslationRequest) -> Result<TranslationResult, AppError> {
    let settings = state.settings().translation;
    if !settings.translation_enabled {
        return Err(AppError::new("disabled", "O serviço de tradução está desativado nas configurações."));
    }
    state.translator.translate(request, &settings).await
}

#[tauri::command]
pub fn get_service_status(state: State<'_, AppState>) -> ServiceStatus {
    let settings = state.settings().translation;
    ServiceStatus {
        translation_enabled: settings.translation_enabled,
        capture_enabled: settings.capture_enabled,
        capture_supported: platform::CAPTURE_SUPPORTED,
    }
}
