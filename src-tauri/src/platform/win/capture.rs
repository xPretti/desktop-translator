//! Captura global de seleção.
//!
//! Estratégia (sem polling e sem monitorar o teclado/clipboard continuamente):
//! 1. Um hook de mouse de baixo nível (WH_MOUSE_LL) apenas observa cliques do botão esquerdo.
//!    O callback é mínimo: classifica o gesto (arrastar ou duplo clique) e envia por canal.
//! 2. Uma thread de trabalho (com COM inicializado) aguarda um breve debounce e lê a
//!    seleção via UI Automation. Cliques injetados (SendInput) e em janelas do próprio app
//!    são ignorados.
//! 3. O atalho global usa UI Automation e, se indisponível, o fallback Ctrl+C com
//!    preservação do clipboard (`capture_now`).

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, SyncSender};
use std::sync::Arc;
use std::thread::{self, sleep};
use std::time::{Duration, Instant};

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::{GetCurrentProcessId, GetCurrentThreadId};
use windows::Win32::UI::Accessibility::{CUIAutomation, IUIAutomation};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, GetDoubleClickTime, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    VIRTUAL_KEY, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetAncestor, GetClassNameW, GetCursorPos, GetForegroundWindow,
    GetMessageW, GetWindowThreadProcessId, PeekMessageW, PostThreadMessageW, SetWindowsHookExW, TranslateMessage,
    UnhookWindowsHookEx, WindowFromPoint, GA_ROOT, HHOOK, LLMHF_INJECTED, MSG, MSLLHOOKSTRUCT, PM_NOREMOVE, WH_MOUSE_LL,
    WM_LBUTTONDOWN, WM_LBUTTONUP, WM_QUIT, WM_USER,
};

use super::{clipboard, uia};
use crate::models::geometry::{CaptureEvent, Rect, SelectionInfo};

const DRAG_THRESHOLD_PX: i32 = 8;
const DEBOUNCE: Duration = Duration::from_millis(90);

#[derive(Clone, Copy, Debug)]
enum MouseEvent {
    Down { x: i32, y: i32 },
    Up { x: i32, y: i32 },
}

#[derive(Default)]
struct HookState {
    sender: Option<SyncSender<MouseEvent>>,
    down: Option<(i32, i32)>,
    last_up: Option<(i32, i32, u32)>,
}

thread_local! {
    static HOOK_STATE: RefCell<HookState> = RefCell::new(HookState::default());
}

fn send(state: &HookState, event: MouseEvent) {
    if let Some(sender) = &state.sender {
        // Nunca bloqueia o hook: se o canal estiver cheio, o evento é descartado.
        let _ = sender.try_send(event);
    }
}

unsafe extern "system" fn mouse_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let info = &*(lparam.0 as *const MSLLHOOKSTRUCT);
        if info.flags & LLMHF_INJECTED == 0 {
            let (x, y) = (info.pt.x, info.pt.y);
            match wparam.0 as u32 {
                WM_LBUTTONDOWN => HOOK_STATE.with(|cell| {
                    let mut state = cell.borrow_mut();
                    state.down = Some((x, y));
                    send(&state, MouseEvent::Down { x, y });
                }),
                WM_LBUTTONUP => HOOK_STATE.with(|cell| {
                    let mut state = cell.borrow_mut();
                    let Some((dx, dy)) = state.down.take() else { return };
                    let dragged = (x - dx).abs().max((y - dy).abs()) >= DRAG_THRESHOLD_PX;
                    let double_click = state.last_up.is_some_and(|(lx, ly, lt)| {
                        info.time.wrapping_sub(lt) <= GetDoubleClickTime() && (x - lx).abs() <= 4 && (y - ly).abs() <= 4
                    });
                    state.last_up = Some((x, y, info.time));
                    if dragged || double_click {
                        send(&state, MouseEvent::Up { x, y });
                    }
                }),
                _ => {}
            }
        }
    }
    CallNextHookEx(HHOOK::default(), code, wparam, lparam)
}

fn run_hook_thread(sender: SyncSender<MouseEvent>, ready: mpsc::Sender<Result<u32, String>>) {
    unsafe {
        HOOK_STATE.with(|cell| cell.borrow_mut().sender = Some(sender));
        let module = GetModuleHandleW(PCWSTR::null()).map(|m| HINSTANCE(m.0)).unwrap_or_default();
        let hook = match SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), module, 0) {
            Ok(hook) => hook,
            Err(error) => {
                let _ = ready.send(Err(format!("Não foi possível instalar o hook de mouse: {error}")));
                return;
            }
        };
        // Garante que a fila de mensagens da thread exista antes de aceitar WM_QUIT.
        let mut msg = MSG::default();
        let _ = PeekMessageW(&mut msg, HWND::default(), WM_USER, WM_USER, PM_NOREMOVE);
        let _ = ready.send(Ok(GetCurrentThreadId()));

        while GetMessageW(&mut msg, HWND::default(), 0, 0).0 > 0 {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        let _ = UnhookWindowsHookEx(hook);
        // Ao soltar o remetente, a thread de trabalho encerra sozinha.
        HOOK_STATE.with(|cell| cell.borrow_mut().sender = None);
    }
}

fn is_own_window(x: i32, y: i32) -> bool {
    unsafe {
        let hwnd = WindowFromPoint(POINT { x, y });
        if hwnd.0.is_null() {
            return false;
        }
        let root = GetAncestor(hwnd, GA_ROOT);
        let mut pid = 0u32;
        GetWindowThreadProcessId(if root.0.is_null() { hwnd } else { root }, Some(&mut pid));
        pid == GetCurrentProcessId()
    }
}

fn create_automation() -> Option<IUIAutomation> {
    unsafe { CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER).ok() }
}

fn run_worker(
    receiver: Receiver<MouseEvent>,
    max_chars: Arc<AtomicUsize>,
    fallback: Arc<AtomicBool>,
    handler: Arc<dyn Fn(CaptureEvent) + Send + Sync>,
) {
    let com_ready = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).is_ok() };
    let automation = create_automation();
    let mut carried: Option<MouseEvent> = None;
    let mut down_in_own_window = false;

    loop {
        let event = match carried.take() {
            Some(event) => event,
            None => match receiver.recv() {
                Ok(event) => event,
                Err(_) => break,
            },
        };
        match event {
            MouseEvent::Down { x, y } => {
                down_in_own_window = is_own_window(x, y);
                if !down_in_own_window {
                    handler(CaptureEvent::OutsideClick);
                }
            }
            MouseEvent::Up { x, y } => {
                if down_in_own_window || is_own_window(x, y) {
                    down_in_own_window = false;
                    continue;
                }
                // Debounce: aguarda o app de origem atualizar a seleção e evita disparos duplicados.
                match receiver.recv_timeout(DEBOUNCE) {
                    Ok(next) => {
                        carried = Some(next);
                        continue;
                    }
                    Err(RecvTimeoutError::Disconnected) => break,
                    Err(RecvTimeoutError::Timeout) => {}
                }
                let limit = max_chars.load(Ordering::Relaxed);
                let mut selection = automation.as_ref().and_then(|a| uia::read_selection(a, limit));
                // Navegadores, PDFs e texto estático costumam não expor TextPattern: tenta copiar.
                if selection.is_none() && fallback.load(Ordering::Relaxed) {
                    selection = clipboard_fallback(limit).ok();
                }
                if let Some(mut selection) = selection {
                    // O botão sempre nasce na posição em que o mouse foi solto.
                    selection.end_rect = Rect::point(x, y);
                    handler(CaptureEvent::Selected(selection));
                }
            }
        }
    }
    if com_ready {
        unsafe { CoUninitialize() };
    }
}

pub struct CaptureHandle {
    hook_thread_id: u32,
}

impl CaptureHandle {
    pub fn start(
        max_chars: Arc<AtomicUsize>,
        fallback: Arc<AtomicBool>,
        handler: Arc<dyn Fn(CaptureEvent) + Send + Sync>,
    ) -> Result<Self, String> {
        let (sender, receiver) = mpsc::sync_channel::<MouseEvent>(64);
        let (ready_tx, ready_rx) = mpsc::channel();
        thread::Builder::new()
            .name("capture-hook".into())
            .spawn(move || run_hook_thread(sender, ready_tx))
            .map_err(|e| e.to_string())?;
        let hook_thread_id = ready_rx
            .recv_timeout(Duration::from_secs(2))
            .map_err(|_| "Tempo esgotado ao instalar o hook de mouse.".to_string())??;
        thread::Builder::new()
            .name("capture-worker".into())
            .spawn(move || run_worker(receiver, max_chars, fallback, handler))
            .map_err(|e| e.to_string())?;
        Ok(Self { hook_thread_id })
    }

    /// Solicita o encerramento. Não aguarda as threads (evita deadlock com a thread principal).
    pub fn stop(self) {
        unsafe {
            let _ = PostThreadMessageW(self.hook_thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
        }
    }
}

pub fn cursor_rect() -> Rect {
    let mut point = POINT::default();
    unsafe {
        let _ = GetCursorPos(&mut point);
    }
    Rect::point(point.x, point.y)
}

// ---------- captura sob demanda (atalho global) ----------

fn key_down(key: VIRTUAL_KEY) -> bool {
    unsafe { (GetAsyncKeyState(i32::from(key.0)) as u16) & 0x8000 != 0 }
}

fn wait_modifiers_released() {
    let deadline = Instant::now() + Duration::from_millis(1500);
    while Instant::now() < deadline
        && [VK_CONTROL, VK_MENU, VK_SHIFT, VK_LWIN, VK_RWIN].iter().any(|k| key_down(*k))
    {
        sleep(Duration::from_millis(10));
    }
}

fn foreground_blocks_copy() -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        let mut buffer = [0u16; 256];
        let length = GetClassNameW(hwnd, &mut buffer);
        if length <= 0 {
            return false;
        }
        let class = String::from_utf16_lossy(&buffer[..length as usize]);
        matches!(
            class.as_str(),
            // Terminais (Ctrl+C interrompe o programa) e shell do Windows (arquivos, desktop, barra de tarefas).
            "ConsoleWindowClass"
                | "CASCADIA_HOSTING_WINDOW_CLASS"
                | "mintty"
                | "VirtualConsoleClass"
                | "CabinetWClass"
                | "Progman"
                | "WorkerW"
                | "Shell_TrayWnd"
        )
    }
}

fn key_input(key: VIRTUAL_KEY, up: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: if up { KEYEVENTF_KEYUP } else { KEYBD_EVENT_FLAGS(0) },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

fn send_ctrl_c() {
    let c = VIRTUAL_KEY(0x43);
    let inputs = [key_input(VK_CONTROL, false), key_input(c, false), key_input(c, true), key_input(VK_CONTROL, true)];
    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

/// Fallback: copia a seleção com Ctrl+C e restaura o clipboard original.
fn clipboard_fallback(max_chars: usize) -> Result<SelectionInfo, String> {
    if foreground_blocks_copy() {
        return Err("Terminais e o Explorador de Arquivos não são suportados para captura por cópia.".into());
    }
    wait_modifiers_released();
    let snapshot = clipboard::snapshot().ok_or("A área de transferência está em uso por outro programa.")?;
    let before = clipboard::sequence();
    send_ctrl_c();

    let mut copied = None;
    for _ in 0..33 {
        sleep(Duration::from_millis(15));
        if clipboard::sequence() != before {
            copied = clipboard::read_text();
            break;
        }
    }
    clipboard::restore(&snapshot);

    let text = copied.map(|t| t.trim().to_string()).filter(|t| !t.is_empty());
    let Some(text) = text else {
        return Err("Nenhum texto selecionado (ou o aplicativo não permite copiar).".into());
    };
    let truncated = text.chars().count() > max_chars;
    let text = if truncated { text.chars().take(max_chars).collect() } else { text };
    let point = cursor_rect();
    Ok(SelectionInfo { text, bounds: point, end_rect: point, truncated })
}

/// Captura a seleção atual: UI Automation primeiro, depois Ctrl+C. Deve rodar em thread própria.
pub fn capture_now(max_chars: usize) -> Result<SelectionInfo, String> {
    let com_ready = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).is_ok() };
    let result = (|| {
        if let Some(automation) = create_automation() {
            if let Some(selection) = uia::read_selection(&automation, max_chars) {
                return Ok(selection);
            }
        }
        clipboard_fallback(max_chars)
    })();
    if com_ready {
        unsafe { CoUninitialize() };
    }
    result
}
