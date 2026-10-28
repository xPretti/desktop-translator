use tauri::{AppHandle, State};

use crate::models::error::AppError;
use crate::services::overlay;
use crate::state::{lock, AppState};

/// Chamado pelo botão flutuante: abre o pop-up com a seleção guardada no backend.
#[tauri::command]
pub fn open_translation_popup(app: AppHandle, state: State<'_, AppState>) -> Result<(), AppError> {
    let pending = lock(&state.pending)
        .take()
        .ok_or_else(|| AppError::new("no_selection", "Nenhuma seleção disponível."))?;
    overlay::open_popup(&app, pending.text, pending.truncated, pending.bounds, None);
    Ok(())
}

#[tauri::command]
pub fn hide_popup(app: AppHandle) {
    overlay::hide_popup(&app);
}

#[tauri::command]
pub fn resize_popup(app: AppHandle, width: f64, height: f64) {
    overlay::resize_popup(&app, width, height);
}

#[tauri::command]
pub fn begin_window_interaction(app: AppHandle, label: String, kind: String) {
    overlay::begin_window_interaction(&app, &label, &kind);
}

#[tauri::command]
pub fn window_control(app: AppHandle, label: String, action: String) {
    overlay::window_control(&app, &label, &action);
}

#[tauri::command]
pub fn open_main_window(app: AppHandle, page: Option<String>) {
    let page = page.filter(|p| matches!(p.as_str(), "translate" | "settings"));
    overlay::show_main(&app, page.as_deref());
}
