use std::ffi::c_void;

use tauri::WebviewWindow;
use windows::Win32::Foundation::{HWND, LPARAM, POINT, RECT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, ReleaseCapture, VK_LBUTTON};
use windows::Win32::UI::WindowsAndMessaging::{
    GetCursorPos, GetWindowLongPtrW, GetWindowRect, PostMessageW, SetWindowLongPtrW, SetWindowPos, ShowWindow, GWL_EXSTYLE,
    HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SW_HIDE, SW_SHOWNOACTIVATE, WM_NCLBUTTONDOWN,
    WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW,
};

/// O HWND do Tauri pode vir de outra versão da crate `windows`; convertemos pelo valor bruto.
fn hwnd_of(window: &WebviewWindow) -> Option<HWND> {
    window.hwnd().ok().map(|h| HWND(h.0 as isize as *mut c_void))
}

/// Faz a janela receber cliques sem ativar (não rouba o foco do app de origem) e sai do Alt+Tab.
pub fn make_non_activating(window: &WebviewWindow) {
    if let Some(hwnd) = hwnd_of(window) {
        unsafe {
            let style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            let extra = (WS_EX_NOACTIVATE.0 | WS_EX_TOOLWINDOW.0) as isize;
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style | extra);
        }
    }
}

/// Exibe a janela por cima das demais sem ativá-la.
pub fn show_no_activate(window: &WebviewWindow) {
    match hwnd_of(window) {
        Some(hwnd) => unsafe {
            let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
            let _ = SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE);
        },
        None => {
            let _ = window.show();
        }
    }
}

/// Oculta pelo Win32. Como `show_no_activate` exibe a janela por fora do Tauri, o estado de
/// visibilidade interno dele fica desatualizado e `window.hide()` não teria efeito.
pub fn hide_window(window: &WebviewWindow) {
    match hwnd_of(window) {
        Some(hwnd) => unsafe {
            let _ = ShowWindow(hwnd, SW_HIDE);
        },
        None => {
            let _ = window.hide();
        }
    }
}

/// Inicia o laço nativo de mover (HTCAPTION = 2) ou redimensionar (ex.: HTBOTTOMRIGHT = 17),
/// como a barra de título/bordas de uma janela comum.
pub fn begin_window_drag(window: &WebviewWindow, hit_test: u32) {
    if let Some(hwnd) = hwnd_of(window) {
        unsafe {
            let _ = ReleaseCapture();
            let mut point = POINT::default();
            let _ = GetCursorPos(&mut point);
            let lparam = LPARAM((((point.y as isize) & 0xFFFF) << 16) | ((point.x as isize) & 0xFFFF));
            let _ = PostMessageW(hwnd, WM_NCLBUTTONDOWN, WPARAM(hit_test as usize), lparam);
        }
    }
}

/// Redimensiona a janela acompanhando o mouse enquanto o botão esquerdo estiver pressionado.
/// Cada quadro aplica posição e tamanho em UMA chamada de `SetWindowPos` (sem o laço modal do
/// Windows), o que evita artefatos em janelas transparentes ao arrastar bordas esquerda/superior.
/// `edge` combina n/s/e/w (ex.: "nw"). O laço dura apenas o arrasto (e no máximo 60 s).
pub fn begin_manual_resize(window: &WebviewWindow, edge: &str, min_logical: (i32, i32)) {
    let Some(hwnd) = hwnd_of(window) else { return };
    let raw = hwnd.0 as isize;
    let edge = edge.to_string();
    std::thread::spawn(move || unsafe {
        let hwnd = HWND(raw as *mut c_void);
        let mut start = RECT::default();
        if GetWindowRect(hwnd, &mut start).is_err() {
            return;
        }
        let mut origin = POINT::default();
        let _ = GetCursorPos(&mut origin);
        let scale = super::monitor::monitor_at(origin.x, origin.y).scale;
        let min_w = (f64::from(min_logical.0) * scale) as i32;
        let min_h = (f64::from(min_logical.1) * scale) as i32;
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);

        while (GetAsyncKeyState(VK_LBUTTON.0 as i32) as u16) & 0x8000 != 0 && std::time::Instant::now() < deadline {
            let mut cursor = POINT::default();
            let _ = GetCursorPos(&mut cursor);
            let (dx, dy) = (cursor.x - origin.x, cursor.y - origin.y);
            let (mut left, mut top, mut right, mut bottom) = (start.left, start.top, start.right, start.bottom);
            if edge.contains('e') {
                right += dx;
            }
            if edge.contains('w') {
                left += dx;
            }
            if edge.contains('s') {
                bottom += dy;
            }
            if edge.contains('n') {
                top += dy;
            }
            if right - left < min_w {
                if edge.contains('w') {
                    left = right - min_w;
                } else {
                    right = left + min_w;
                }
            }
            if bottom - top < min_h {
                if edge.contains('n') {
                    top = bottom - min_h;
                } else {
                    bottom = top + min_h;
                }
            }
            // Compara com o retângulo real: se algo alterou a janela por fora, o laço reaplica.
            let mut current = RECT::default();
            let _ = GetWindowRect(hwnd, &mut current);
            let next = (left, top, right - left, bottom - top);
            let actual = (current.left, current.top, current.right - current.left, current.bottom - current.top);
            if next != actual {
                let _ = SetWindowPos(hwnd, HWND::default(), next.0, next.1, next.2, next.3, SWP_NOACTIVATE | SWP_NOZORDER);
            }
            std::thread::sleep(std::time::Duration::from_millis(8));
        }
    });
}
