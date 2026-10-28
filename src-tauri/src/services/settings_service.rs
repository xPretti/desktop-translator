//! Aplica configurações: persistência e efeitos colaterais (autostart, atalhos, captura, bandeja).

use std::sync::atomic::Ordering;
use std::sync::Arc;

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_autostart::ManagerExt;

use crate::models::error::AppError;
use crate::models::geometry::CaptureEvent;
use crate::models::settings::{SaveResult, Settings};
use crate::platform::{self, CaptureHandle};
use crate::services::{overlay, selection, shortcuts, tray};
use crate::state::{lock, AppState};

fn sync_autostart(app: &AppHandle, settings: &Settings, warnings: &mut Vec<String>) {
    let manager = app.autolaunch();
    let enabled = manager.is_enabled().unwrap_or(false);
    let want = settings.behavior.launch_at_startup;
    if enabled == want {
        return;
    }
    let result = if want { manager.enable() } else { manager.disable() };
    if result.is_err() {
        warnings.push("Não foi possível alterar a inicialização com o Windows.".into());
    }
}

fn sync_capture(app: &AppHandle, settings: &Settings, warnings: &mut Vec<String>) {
    let state = app.state::<AppState>();
    state.capture_max_chars.store(settings.translation.max_text_length as usize, Ordering::Relaxed);
    state.capture_fallback.store(settings.translation.clipboard_fallback, Ordering::Relaxed);
    let want = settings.translation.capture_enabled && settings.translation.translation_enabled && platform::CAPTURE_SUPPORTED;
    let mut guard = lock(&state.capture);
    match (want, guard.is_some()) {
        (true, false) => {
            let handle_app = app.clone();
            let handler = Arc::new(move |event: CaptureEvent| selection::handle_capture_event(&handle_app, event));
            match CaptureHandle::start(state.capture_max_chars.clone(), state.capture_fallback.clone(), handler) {
                Ok(handle) => *guard = Some(handle),
                Err(message) => warnings.push(message),
            }
        }
        (false, true) => {
            if let Some(handle) = guard.take() {
                handle.stop();
            }
            drop(guard);
            *lock(&state.pending) = None;
            overlay::hide_button(app);
        }
        _ => {}
    }
}

/// Aplica efeitos colaterais sem persistir (usado na inicialização).
pub fn apply_runtime(app: &AppHandle, settings: &Settings) -> Vec<String> {
    let mut warnings = Vec::new();
    sync_autostart(app, settings, &mut warnings);
    shortcuts::register(app, settings, &mut warnings);
    sync_capture(app, settings, &mut warnings);
    tray::sync(app, settings);
    warnings
}

pub fn apply(app: &AppHandle, incoming: Settings) -> Result<SaveResult, AppError> {
    let settings = incoming.sanitized();
    let state = app.state::<AppState>();
    state.store.save(&settings)?;
    state.set_settings(settings.clone());
    let warnings = apply_runtime(app, &settings);
    let _ = app.emit("settings-changed", &settings);
    Ok(SaveResult { settings, warnings })
}

pub fn toggle_capture(app: &AppHandle) {
    let mut settings = app.state::<AppState>().settings();
    settings.translation.capture_enabled = !settings.translation.capture_enabled;
    let _ = apply(app, settings);
}
