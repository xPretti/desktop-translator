use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::Arc;

use tauri::WebviewWindow;

use crate::models::geometry::{CaptureEvent, MonitorInfo, Rect, SelectionInfo};

pub const CAPTURE_SUPPORTED: bool = false;

pub struct CaptureHandle;

impl CaptureHandle {
    pub fn start(
        _max_chars: Arc<AtomicUsize>,
        _fallback: Arc<AtomicBool>,
        _handler: Arc<dyn Fn(CaptureEvent) + Send + Sync>,
    ) -> Result<Self, String> {
        Err("A captura global de seleção só está disponível no Windows.".into())
    }

    pub fn stop(self) {}
}

pub fn capture_now(_max_chars: usize) -> Result<SelectionInfo, String> {
    Err("A captura de seleção só está disponível no Windows.".into())
}

pub fn cursor_rect() -> Rect {
    Rect::point(0, 0)
}

pub fn monitor_at(_x: i32, _y: i32) -> MonitorInfo {
    MonitorInfo { work: Rect { left: 0, top: 0, right: 1920, bottom: 1080 }, scale: 1.0 }
}

pub fn copy_text(_text: &str) -> Result<(), String> {
    Err("A área de transferência só é suportada no Windows.".into())
}

pub fn make_non_activating(_window: &WebviewWindow) {}

pub fn show_no_activate(window: &WebviewWindow) {
    let _ = window.show();
}

pub fn hide_window(window: &WebviewWindow) {
    let _ = window.hide();
}

pub fn begin_window_drag(_window: &WebviewWindow, _hit_test: u32) {}

pub fn begin_manual_resize(_window: &WebviewWindow, _edge: &str, _min_logical: (i32, i32)) {}
