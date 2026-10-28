use std::mem::size_of;

use windows::Win32::Foundation::POINT;
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONEAREST};
use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};

use crate::models::geometry::{MonitorInfo, Rect};

/// Monitor mais próximo do ponto (pixels físicos): área útil (sem barra de tarefas) e escala de DPI.
pub fn monitor_at(x: i32, y: i32) -> MonitorInfo {
    unsafe {
        let monitor = MonitorFromPoint(POINT { x, y }, MONITOR_DEFAULTTONEAREST);
        let mut info = MONITORINFO { cbSize: size_of::<MONITORINFO>() as u32, ..Default::default() };
        let work = if GetMonitorInfoW(monitor, &mut info).as_bool() {
            Rect {
                left: info.rcWork.left,
                top: info.rcWork.top,
                right: info.rcWork.right,
                bottom: info.rcWork.bottom,
            }
        } else {
            Rect { left: 0, top: 0, right: 1920, bottom: 1080 }
        };
        let (mut dpi_x, mut dpi_y) = (96u32, 96u32);
        let scale = match GetDpiForMonitor(monitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y) {
            Ok(()) if dpi_x > 0 => f64::from(dpi_x) / 96.0,
            _ => 1.0,
        };
        MonitorInfo { work, scale }
    }
}
