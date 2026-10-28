use tauri::{AppHandle, State};

use crate::models::error::AppError;
use crate::models::settings::{SaveResult, Settings};
use crate::services::settings_service;
use crate::state::AppState;

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Settings {
    state.settings()
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: Settings) -> Result<SaveResult, AppError> {
    settings_service::apply(&app, settings)
}

#[tauri::command]
pub fn reset_settings(app: AppHandle) -> Result<SaveResult, AppError> {
    settings_service::apply(&app, Settings::default())
}
