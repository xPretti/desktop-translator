use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::models::settings::Settings;
use crate::services::{overlay, selection, settings_service};
use crate::state::{lock, AppState, ShortcutAction};

pub fn plugin() -> TauriPlugin<Wry> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_handler(|app, shortcut, event| {
            if event.state() != ShortcutState::Pressed {
                return;
            }
            let action = {
                let state = app.state::<AppState>();
                let table = lock(&state.shortcuts);
                table.iter().find(|(s, _)| s == shortcut).map(|(_, a)| *a)
            };
            match action {
                Some(ShortcutAction::ToggleCapture) => settings_service::toggle_capture(app),
                Some(ShortcutAction::OpenMain) => overlay::show_main(app, None),
                Some(ShortcutAction::TranslateSelection) => selection::translate_selection_now(app),
                None => {}
            }
        })
        .build()
}

/// Registra os atalhos configurados; atalhos inválidos ou em uso viram avisos, não erros fatais.
pub fn register(app: &AppHandle, settings: &Settings, warnings: &mut Vec<String>) {
    let manager = app.global_shortcut();
    let _ = manager.unregister_all();
    let mut table = Vec::new();
    let wanted = [
        (&settings.behavior.shortcuts.translate_selection, ShortcutAction::TranslateSelection, "traduzir seleção"),
        (&settings.behavior.shortcuts.toggle_capture, ShortcutAction::ToggleCapture, "ativar/desativar captura"),
        (&settings.behavior.shortcuts.open_main, ShortcutAction::OpenMain, "abrir janela principal"),
    ];
    for (accelerator, action, label) in wanted {
        if accelerator.is_empty() {
            continue;
        }
        match accelerator.parse::<Shortcut>() {
            Ok(shortcut) => match manager.register(accelerator.as_str()) {
                Ok(()) => table.push((shortcut, action)),
                Err(_) => warnings.push(format!("Atalho \"{accelerator}\" ({label}) não pôde ser registrado; talvez esteja em uso.")),
            },
            Err(_) => warnings.push(format!("Atalho \"{accelerator}\" ({label}) é inválido.")),
        }
    }
    *lock(&app.state::<AppState>().shortcuts) = table;
}
