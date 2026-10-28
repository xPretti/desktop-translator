//! Implementação Windows 10/11.
//!
//! - `capture`: hook de mouse de baixo nível (somente eventos, sem polling) + leitura da
//!   seleção via UI Automation (`uia`) e fallback opcional por Ctrl+C (`clipboard`).
//! - `monitor`: área útil e DPI por monitor. - `window_style`: janelas que não roubam foco.

mod capture;
mod clipboard;
mod monitor;
mod uia;
mod window_style;

pub use capture::{capture_now, cursor_rect, CaptureHandle};
pub use clipboard::copy_text;
pub use monitor::monitor_at;
pub use window_style::{begin_manual_resize, begin_window_drag, hide_window, make_non_activating, show_no_activate};

pub const CAPTURE_SUPPORTED: bool = true;
