//! Cola entre a captura nativa e as janelas flutuantes.

use tauri::{AppHandle, Manager};

use crate::models::geometry::CaptureEvent;
use crate::models::settings::NewSelectionBehavior;
use crate::platform;
use crate::services::overlay;
use crate::state::{lock, AppState, PendingSelection};

pub fn handle_capture_event(app: &AppHandle, event: CaptureEvent) {
    let state = app.state::<AppState>();
    let settings = state.settings();
    match event {
        CaptureEvent::OutsideClick => {
            *lock(&state.pending) = None;
            overlay::hide_button(app);
            if settings.behavior.close_on_outside_click {
                overlay::hide_popup(app);
            }
        }
        CaptureEvent::Selected(selection) => {
            if !settings.translation.translation_enabled || !settings.translation.capture_enabled {
                return;
            }
            if settings.behavior.new_selection_behavior == NewSelectionBehavior::Close {
                overlay::hide_popup(app);
            }
            let button_visible = app.get_webview_window(overlay::BUTTON).and_then(|w| w.is_visible().ok()).unwrap_or(false);
            {
                let mut pending = lock(&state.pending);
                if button_visible
                    && pending.as_ref().is_some_and(|p| p.text == selection.text && p.bounds == selection.bounds)
                {
                    return; // mesma seleção já exibida: evita disparo duplicado
                }
                *pending = Some(PendingSelection {
                    text: selection.text.clone(),
                    bounds: selection.bounds,
                    truncated: selection.truncated,
                });
            }
            if settings.translation.auto_translate {
                *lock(&state.pending) = None;
                overlay::open_popup(app, selection.text, selection.truncated, selection.bounds, None);
            } else {
                overlay::show_button(app, &selection);
            }
        }
    }
}

/// Atalho global: captura sob demanda (UI Automation ou Ctrl+C) e abre o pop-up.
pub fn translate_selection_now(app: &AppHandle) {
    let state = app.state::<AppState>();
    let settings = state.settings();
    if !settings.translation.translation_enabled {
        return;
    }
    let max_chars = settings.translation.max_text_length as usize;
    let handle = app.clone();
    std::thread::spawn(move || match platform::capture_now(max_chars) {
        Ok(selection) => overlay::open_popup(&handle, selection.text, selection.truncated, selection.bounds, None),
        Err(message) => overlay::open_popup(&handle, String::new(), false, platform::cursor_rect(), Some(message)),
    });
}
