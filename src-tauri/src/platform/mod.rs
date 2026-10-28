//! Integração nativa. No Windows usa Win32/UI Automation; em outros sistemas há apenas
//! stubs que permitem compilar e testar a lógica independente de plataforma.

#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use win::*;

#[cfg(not(windows))]
mod stub;
#[cfg(not(windows))]
pub use stub::*;
