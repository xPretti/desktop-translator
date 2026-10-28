use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

use crate::models::settings::Settings;
use crate::services::{overlay, settings_service};
use crate::state::{lock, AppState};

pub fn build(app: &AppHandle) -> tauri::Result<()> {
    let state = app.state::<AppState>();
    let capture_on = state.settings().translation.capture_enabled;

    let open = MenuItem::with_id(app, "open", "Abrir Desktop Translator", true, None::<&str>)?;
    let toggle = CheckMenuItem::with_id(app, "toggle", "Tradução por seleção", true, capture_on, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Configurações", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Encerrar", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &toggle, &settings, &separator, &quit])?;
    *lock(&state.tray_toggle) = Some(toggle);

    let mut builder = TrayIconBuilder::with_id("main-tray")
        .tooltip("Desktop Translator")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => overlay::show_main(app, Some("translate")),
            "settings" => overlay::show_main(app, Some("settings")),
            "toggle" => settings_service::toggle_capture(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                overlay::show_main(tray.app_handle(), None);
            }
        });
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(())
}

/// Mantém o item de menu sincronizado com a configuração.
pub fn sync(app: &AppHandle, settings: &Settings) {
    if let Some(item) = lock(&app.state::<AppState>().tray_toggle).as_ref() {
        let _ = item.set_checked(settings.translation.capture_enabled);
    }
}
