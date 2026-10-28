//! Leitura da seleção via Microsoft UI Automation (TextPattern).
//! Funciona em apps que expõem o padrão de texto (Notepad, Word, Edge/WebView2 com
//! acessibilidade ativa, campos Win32/WPF etc.). Campos de senha são ignorados.

use std::ffi::c_void;

use windows::Win32::System::Com::SAFEARRAY;
use windows::Win32::System::Ole::{SafeArrayAccessData, SafeArrayDestroy, SafeArrayGetLBound, SafeArrayGetUBound, SafeArrayUnaccessData};
use windows::Win32::UI::Accessibility::{IUIAutomation, IUIAutomationTextPattern, UIA_TextPatternId};

use crate::models::geometry::{Rect, SelectionInfo};

const MAX_RANGES: i32 = 5;

/// Converte o SAFEARRAY de `double` (esquerda, topo, largura, altura)* em retângulos.
unsafe fn read_rects(psa: *mut SAFEARRAY) -> Vec<Rect> {
    if psa.is_null() {
        return Vec::new();
    }
    let lower = SafeArrayGetLBound(psa, 1).unwrap_or(0);
    let upper = SafeArrayGetUBound(psa, 1).unwrap_or(-1);
    let mut values: Vec<f64> = Vec::new();
    if upper >= lower {
        let mut data: *mut c_void = std::ptr::null_mut();
        if SafeArrayAccessData(psa, &mut data).is_ok() && !data.is_null() {
            let count = (upper - lower + 1) as usize;
            values = std::slice::from_raw_parts(data as *const f64, count).to_vec();
            let _ = SafeArrayUnaccessData(psa);
        }
    }
    let _ = SafeArrayDestroy(psa);
    values
        .chunks_exact(4)
        .map(|c| Rect {
            left: c[0].floor() as i32,
            top: c[1].floor() as i32,
            right: (c[0] + c[2]).ceil() as i32,
            bottom: (c[1] + c[3]).ceil() as i32,
        })
        .collect()
}

pub fn read_selection(uia: &IUIAutomation, max_chars: usize) -> Option<SelectionInfo> {
    unsafe {
        let focused = uia.GetFocusedElement().ok()?;
        if focused.CurrentIsPassword().map(|b| b.as_bool()).unwrap_or(false) {
            return None;
        }
        let pattern: IUIAutomationTextPattern = focused.GetCurrentPatternAs(UIA_TextPatternId).ok()?;
        let ranges = pattern.GetSelection().ok()?;
        let count = ranges.Length().ok()?.min(MAX_RANGES);
        if count <= 0 {
            return None;
        }

        let mut text = String::new();
        let mut bounds: Option<Rect> = None;
        let mut last: Option<Rect> = None;
        for index in 0..count {
            let range = ranges.GetElement(index).ok()?;
            let part = range.GetText((max_chars + 1) as i32).ok()?.to_string();
            if !text.is_empty() && !part.is_empty() {
                text.push('\n');
            }
            text.push_str(&part);
            if let Ok(psa) = range.GetBoundingRectangles() {
                for rect in read_rects(psa) {
                    bounds = Some(bounds.map_or(rect, |b| b.union(rect)));
                    last = Some(rect);
                }
            }
        }

        let text = text.trim().to_string();
        if text.is_empty() {
            return None;
        }
        let truncated = text.chars().count() > max_chars;
        let text: String = if truncated { text.chars().take(max_chars).collect() } else { text };
        let fallback = super::capture::cursor_rect();
        let bounds = bounds.unwrap_or(fallback);
        Some(SelectionInfo { text, bounds, end_rect: last.unwrap_or(bounds), truncated })
    }
}
