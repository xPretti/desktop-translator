use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize};
use std::sync::{Arc, Mutex, RwLock};

use tauri::menu::CheckMenuItem;
use tauri::Wry;
use tauri_plugin_global_shortcut::Shortcut;

use crate::models::geometry::Rect;
use crate::models::settings::Settings;
use crate::platform::CaptureHandle;
use crate::services::translation::TranslationService;
use crate::settings::SettingsStore;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShortcutAction {
    ToggleCapture,
    OpenMain,
    TranslateSelection,
}

/// Seleção detectada aguardando o clique no botão. O texto permanece apenas em memória no backend.
pub struct PendingSelection {
    pub text: String,
    pub bounds: Rect,
    pub truncated: bool,
}

pub struct AppState {
    settings: RwLock<Settings>,
    pub store: SettingsStore,
    pub translator: TranslationService,
    pub pending: Mutex<Option<PendingSelection>>,
    pub popup_anchor: Mutex<Option<Rect>>,
    /// O usuário moveu/redimensionou o pop-up: não reposicionar automaticamente.
    pub popup_pinned: AtomicBool,
    /// Último tamanho lógico (px CSS) informado pelo pop-up.
    pub popup_size: Mutex<(f64, f64)>,
    pub capture: Mutex<Option<CaptureHandle>>,
    pub capture_max_chars: Arc<AtomicUsize>,
    pub capture_fallback: Arc<AtomicBool>,
    pub shortcuts: Mutex<Vec<(Shortcut, ShortcutAction)>>,
    pub tray_toggle: Mutex<Option<CheckMenuItem<Wry>>>,
    pub button_generation: Arc<AtomicU64>,
    pub request_counter: AtomicU64,
}

impl AppState {
    pub fn new(store: SettingsStore, settings: Settings) -> Self {
        Self {
            settings: RwLock::new(settings),
            store,
            translator: TranslationService::new(),
            pending: Mutex::new(None),
            popup_anchor: Mutex::new(None),
            popup_pinned: AtomicBool::new(false),
            popup_size: Mutex::new((460.0, 140.0)),
            capture: Mutex::new(None),
            capture_max_chars: Arc::new(AtomicUsize::new(3000)),
            capture_fallback: Arc::new(AtomicBool::new(true)),
            shortcuts: Mutex::new(Vec::new()),
            tray_toggle: Mutex::new(None),
            button_generation: Arc::new(AtomicU64::new(0)),
            request_counter: AtomicU64::new(0),
        }
    }

    pub fn settings(&self) -> Settings {
        self.settings.read().unwrap_or_else(|e| e.into_inner()).clone()
    }

    pub fn set_settings(&self, value: Settings) {
        *self.settings.write().unwrap_or_else(|e| e.into_inner()) = value;
    }
}

/// Bloqueia um `Mutex` ignorando envenenamento (o estado protegido é sempre consistente).
pub fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|e| e.into_inner())
}
