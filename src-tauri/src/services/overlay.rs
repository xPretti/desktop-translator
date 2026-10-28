//! Janelas flutuantes (botão e pop-up): criação, posicionamento e exibição sem roubar foco.

use std::sync::atomic::Ordering;
use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

use crate::models::geometry::{Rect, SelectionInfo, Size};
use crate::models::translation::PopupPayload;
use crate::platform;
use crate::services::placement::{place, PlacementPrefs};
use crate::state::{lock, AppState};

pub const MAIN: &str = "main";
pub const BUTTON: &str = "floating-button";
pub const POPUP: &str = "popup";

/// Tamanho lógico da janela do botão (igual ao botão, sem margem).
const BUTTON_LOGICAL: f64 = 32.0;
const POPUP_INITIAL: (f64, f64) = (460.0, 140.0);

pub fn create_windows(app: &AppHandle) -> tauri::Result<()> {
    for (label, (width, height)) in [(BUTTON, (BUTTON_LOGICAL, BUTTON_LOGICAL)), (POPUP, POPUP_INITIAL)] {
        let window = WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
            .title("Desktop Translator")
            .inner_size(width, height)
            .decorations(false)
            .transparent(true)
            .shadow(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .resizable(label == POPUP)
            .focused(false)
            .visible(false)
            .build()?;
        platform::make_non_activating(&window);
    }
    Ok(())
}

/// Posiciona a janela (físico) e a dimensiona a partir de um tamanho lógico do CSS.
fn place_window(window: &WebviewWindow, anchor: Rect, logical: (f64, f64), prefs: &PlacementPrefs) {
    let (cx, cy) = anchor.center();
    let monitor = platform::monitor_at(cx, cy);
    let max_w = (monitor.work.width() - 2 * prefs.margin).max(50);
    let max_h = (monitor.work.height() - 2 * prefs.margin).max(50);
    let size = Size {
        width: ((logical.0 * monitor.scale).round() as i32).clamp(1, max_w),
        height: ((logical.1 * monitor.scale).round() as i32).clamp(1, max_h),
    };
    let (x, y) = place(anchor, size, monitor.work, prefs);
    // Posição antes do tamanho: ao cruzar monitores com DPI diferente, o Windows reajusta a janela.
    let _ = window.set_position(PhysicalPosition::new(x, y));
    let _ = window.set_size(PhysicalSize::new(size.width as u32, size.height as u32));
}

pub fn show_button(app: &AppHandle, selection: &SelectionInfo) {
    let state = app.state::<AppState>();
    let settings = state.settings();
    let Some(window) = app.get_webview_window(BUTTON) else { return };
    let end = selection.end_rect;
    // O botão é ancorado à extremidade final da seleção.
    let anchor = Rect { left: end.right, right: end.right, top: end.top, bottom: end.bottom };
    place_window(&window, anchor, (BUTTON_LOGICAL, BUTTON_LOGICAL), &PlacementPrefs::button(&settings.positioning));
    platform::show_no_activate(&window);

    let generation = state.button_generation.fetch_add(1, Ordering::SeqCst) + 1;
    let seconds = settings.behavior.button_auto_hide_seconds;
    if seconds > 0 {
        let handle = app.clone();
        let counter = state.button_generation.clone();
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(Duration::from_secs(seconds as u64)).await;
            if counter.load(Ordering::SeqCst) == generation {
                hide_button(&handle);
            }
        });
    }
}

pub fn hide_button(app: &AppHandle) {
    let state = app.state::<AppState>();
    state.button_generation.fetch_add(1, Ordering::SeqCst);
    if let Some(window) = app.get_webview_window(BUTTON) {
        platform::hide_window(&window);
    }
}

fn apply_popup_geometry(app: &AppHandle) {
    let state = app.state::<AppState>();
    let Some(anchor) = *lock(&state.popup_anchor) else { return };
    let size = *lock(&state.popup_size);
    let settings = state.settings();
    let Some(window) = app.get_webview_window(POPUP) else { return };
    if state.popup_pinned.load(Ordering::SeqCst) {
        if window.is_maximized().unwrap_or(false) {
            return;
        }
        // Posição escolhida pelo usuário: só acompanha o tamanho do conteúdo.
        let (x, y) = window.outer_position().map(|p| (p.x, p.y)).unwrap_or((0, 0));
        let scale = platform::monitor_at(x, y).scale;
        let _ = window.set_size(PhysicalSize::new(
            ((size.0 * scale).round() as u32).max(1),
            ((size.1 * scale).round() as u32).max(1),
        ));
        return;
    }
    place_window(&window, anchor, size, &PlacementPrefs::popup(&settings.positioning));
}

/// Abre o pop-up junto à seleção e envia o texto (ou um erro) ao frontend.
pub fn open_popup(app: &AppHandle, text: String, truncated: bool, anchor: Rect, error: Option<String>) {
    hide_button(app);
    let state = app.state::<AppState>();
    *lock(&state.popup_anchor) = Some(anchor);
    state.popup_pinned.store(false, Ordering::SeqCst);
    apply_popup_geometry(app);
    if let Some(window) = app.get_webview_window(POPUP) {
        platform::show_no_activate(&window);
    }
    let request_id = state.request_counter.fetch_add(1, Ordering::SeqCst) + 1;
    let _ = app.emit_to(POPUP, "popup-show", PopupPayload { text, truncated, request_id, error });
}

pub fn resize_popup(app: &AppHandle, width: f64, height: f64) {
    if !width.is_finite() || !height.is_finite() {
        return;
    }
    let state = app.state::<AppState>();
    *lock(&state.popup_size) = (width.clamp(80.0, 1800.0), height.clamp(40.0, 1600.0));
    let visible = app.get_webview_window(POPUP).and_then(|w| w.is_visible().ok()).unwrap_or(false);
    if visible {
        apply_popup_geometry(app);
    }
}

pub fn hide_popup(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(POPUP) {
        platform::hide_window(&window);
    }
}

/// Ação nativa de janela: "move" ou "resize-{n,s,e,w,ne,nw,se,sw}" (laço do Windows, como bordas e barra de título).
pub fn begin_window_interaction(app: &AppHandle, label: &str, kind: &str) {
    if label != MAIN && label != POPUP {
        return;
    }
    // O pop-up (transparente) usa laço próprio; a janela principal usa o redimensionamento nativo.
    if label == POPUP {
        if let Some(edge) = kind.strip_prefix("resize-") {
            if matches!(edge, "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw") {
                app.state::<AppState>().popup_pinned.store(true, Ordering::SeqCst);
                if let Some(window) = app.get_webview_window(label) {
                    platform::begin_manual_resize(&window, edge, (200, 90));
                }
            }
            return;
        }
    }
    let hit = match kind {
        "move" => 2, // HTCAPTION
        "resize-w" => 10,
        "resize-e" => 11,
        "resize-n" => 12,
        "resize-nw" => 13,
        "resize-ne" => 14,
        "resize-s" => 15,
        "resize-sw" => 16,
        "resize-se" => 17,
        _ => return,
    };
    if label == POPUP {
        app.state::<AppState>().popup_pinned.store(true, Ordering::SeqCst);
    }
    if let Some(window) = app.get_webview_window(label) {
        platform::begin_window_drag(&window, hit);
    }
}

/// Controles de janela customizados. O estado de maximizado volta ao frontend pelo evento `window-maximized`.
pub fn window_control(app: &AppHandle, label: &str, action: &str) {
    if label != MAIN && label != POPUP {
        return;
    }
    let Some(window) = app.get_webview_window(label) else { return };
    match action {
        "minimize" if label == MAIN => {
            let _ = window.minimize();
        }
        "toggle_maximize" => {
            if label == POPUP {
                app.state::<AppState>().popup_pinned.store(true, Ordering::SeqCst);
            }
            if window.is_maximized().unwrap_or(false) {
                let _ = window.unmaximize();
            } else {
                let _ = window.maximize();
            }
        }
        "close" => {
            if label == MAIN {
                let _ = window.close(); // passa pela regra de fechar (bandeja ou encerrar)
            } else {
                hide_popup(app);
            }
        }
        _ => {}
    }
}

pub fn hide_all(app: &AppHandle) {
    hide_button(app);
    hide_popup(app);
}

pub fn show_main(app: &AppHandle, page: Option<&str>) {
    if let Some(window) = app.get_webview_window(MAIN) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        if let Some(page) = page {
            let _ = app.emit_to(MAIN, "navigate", page);
        }
    }
}
