//! Acesso à área de transferência. O snapshot guarda todos os formatos baseados em memória
//! (texto, imagens DIB, HTML, arquivos etc.) para restaurar o conteúdo original após o
//! fallback por Ctrl+C. Formatos baseados em handles GDI (CF_BITMAP etc.) não são preservados.

use std::ffi::c_void;
use std::thread::sleep;
use std::time::Duration;

use windows::Win32::Foundation::{GlobalFree, HANDLE, HGLOBAL, HWND};
use windows::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, EnumClipboardFormats, GetClipboardData, GetClipboardSequenceNumber,
    OpenClipboard, SetClipboardData,
};
use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock, GMEM_MOVEABLE};

const CF_UNICODETEXT: u32 = 13;

struct ClipboardGuard;

impl ClipboardGuard {
    fn open() -> Option<Self> {
        for _ in 0..10 {
            if unsafe { OpenClipboard(HWND::default()) }.is_ok() {
                return Some(Self);
            }
            sleep(Duration::from_millis(10));
        }
        None
    }
}

impl Drop for ClipboardGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseClipboard();
        }
    }
}

pub struct Snapshot(Vec<(u32, Vec<u8>)>);

fn is_handle_based(format: u32) -> bool {
    matches!(format, 2 | 3 | 9 | 14 | 0x0080..=0x008E | 0x0200..=0x03FF)
}

pub fn sequence() -> u32 {
    unsafe { GetClipboardSequenceNumber() }
}

pub fn snapshot() -> Option<Snapshot> {
    let _guard = ClipboardGuard::open()?;
    let mut items = Vec::new();
    let mut format = 0u32;
    loop {
        format = unsafe { EnumClipboardFormats(format) };
        if format == 0 {
            break;
        }
        if is_handle_based(format) {
            continue;
        }
        unsafe {
            let Ok(handle) = GetClipboardData(format) else { continue };
            let global = HGLOBAL(handle.0);
            let size = GlobalSize(global);
            if size == 0 {
                continue;
            }
            let ptr = GlobalLock(global);
            if ptr.is_null() {
                continue;
            }
            let bytes = std::slice::from_raw_parts(ptr as *const u8, size).to_vec();
            let _ = GlobalUnlock(global);
            items.push((format, bytes));
        }
    }
    Some(Snapshot(items))
}

unsafe fn set_bytes(format: u32, bytes: &[u8]) -> bool {
    let Ok(global) = GlobalAlloc(GMEM_MOVEABLE, bytes.len()) else { return false };
    let ptr = GlobalLock(global);
    if ptr.is_null() {
        let _ = GlobalFree(global);
        return false;
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr as *mut u8, bytes.len());
    let _ = GlobalUnlock(global);
    if SetClipboardData(format, HANDLE(global.0 as *mut c_void)).is_err() {
        let _ = GlobalFree(global);
        return false;
    }
    true
}

pub fn restore(snapshot: &Snapshot) {
    let Some(_guard) = ClipboardGuard::open() else { return };
    unsafe {
        let _ = EmptyClipboard();
        for (format, bytes) in &snapshot.0 {
            set_bytes(*format, bytes);
        }
    }
}

pub fn read_text() -> Option<String> {
    let _guard = ClipboardGuard::open()?;
    unsafe {
        let handle = GetClipboardData(CF_UNICODETEXT).ok()?;
        let global = HGLOBAL(handle.0);
        let units = GlobalSize(global) / 2;
        let ptr = GlobalLock(global) as *const u16;
        if ptr.is_null() {
            return None;
        }
        let slice = std::slice::from_raw_parts(ptr, units);
        let end = slice.iter().position(|&c| c == 0).unwrap_or(units);
        let text = String::from_utf16_lossy(&slice[..end]);
        let _ = GlobalUnlock(global);
        Some(text)
    }
}

pub fn copy_text(text: &str) -> Result<(), String> {
    let mut bytes: Vec<u8> = text.encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
    bytes.extend_from_slice(&[0, 0]);
    let _guard = ClipboardGuard::open().ok_or("A área de transferência está em uso por outro programa.")?;
    unsafe {
        let _ = EmptyClipboard();
        if set_bytes(CF_UNICODETEXT, &bytes) {
            Ok(())
        } else {
            Err("Não foi possível copiar para a área de transferência.".into())
        }
    }
}
