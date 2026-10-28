mod commands;
mod models;
mod platform;
mod services;
mod settings;
mod state;

use tauri::{Emitter, Manager, RunEvent, Window, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;

use crate::models::settings::CloseAction;
use crate::services::{overlay, settings_service, tray};
use crate::settings::SettingsStore;
use crate::state::{lock, AppState};

/// Eventos das janelas próprias. Na principal: fechar ≠ minimizar ≠ encerrar o processo.
fn on_window_event(window: &Window, event: &WindowEvent) {
    let label = window.label();
    if label != overlay::MAIN && label != overlay::POPUP {
        return;
    }
    let app = window.app_handle();
    if let WindowEvent::Resized(_) = event {
        let _ = app.emit_to(label, "window-maximized", window.is_maximized().unwrap_or(false));
    }
    if label != overlay::MAIN {
        return;
    }
    let settings = app.state::<AppState>().settings();
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            match settings.behavior.close_action {
                CloseAction::Tray => {
                    let _ = window.hide();
                }
                CloseAction::Quit => app.exit(0),
            }
        }
        WindowEvent::Resized(_) => {
            if settings.behavior.minimize_to_tray && window.is_minimized().unwrap_or(false) {
                let _ = window.hide();
            }
        }
        _ => {}
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            overlay::show_main(app, None);
        }))
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(services::shortcuts::plugin())
        .setup(|app| {
            let handle = app.handle().clone();
            let dir = app.path().app_config_dir()?;
            let store = SettingsStore::new(&dir);
            let settings = store.load();
            app.manage(AppState::new(store, settings.clone()));

            overlay::create_windows(&handle)?;
            tray::build(&handle)?;
            let warnings = settings_service::apply_runtime(&handle, &settings);
            for warning in warnings {
                eprintln!("[aviso] {warning}");
            }

            if !settings.behavior.start_minimized {
                overlay::show_main(&handle, None);
            }
            Ok(())
        })
        .on_window_event(on_window_event)
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::settings::reset_settings,
            commands::translation::translate,
            commands::translation::get_service_status,
            commands::overlay::open_translation_popup,
            commands::overlay::hide_popup,
            commands::overlay::resize_popup,
            commands::overlay::begin_window_interaction,
            commands::overlay::window_control,
            commands::overlay::open_main_window,
            commands::system::copy_text,
            commands::system::quit_app,
        ])
        .build(tauri::generate_context!())
        .expect("falha ao construir o aplicativo Tauri")
        .run(|app, event| {
            if let RunEvent::Exit = event {
                // Libera o hook de mouse e demais recursos nativos.
                if let Some(state) = app.try_state::<AppState>() {
                    if let Some(handle) = lock(&state.capture).take() {
                        handle.stop();
                    }
                }
            }
        });
}
