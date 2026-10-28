use tauri::AppHandle;

use crate::models::error::AppError;
use crate::platform;

const MAX_COPY_CHARS: usize = 100_000;

#[tauri::command]
pub fn copy_text(text: String) -> Result<(), AppError> {
    if text.chars().count() > MAX_COPY_CHARS {
        return Err(AppError::new("invalid_request", "Texto grande demais para copiar."));
    }
    platform::copy_text(&text).map_err(|message| AppError::new("clipboard", message))
}

#[tauri::command]
pub fn quit_app(app: AppHandle) {
    app.exit(0);
}
