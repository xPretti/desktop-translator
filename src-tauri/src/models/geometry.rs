/// Retângulo em pixels físicos de tela (coordenadas virtuais do Windows).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn point(x: i32, y: i32) -> Self {
        Self { left: x, top: y, right: x, bottom: y }
    }

    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.left + self.right) / 2, (self.top + self.bottom) / 2)
    }

    pub fn union(self, other: Rect) -> Rect {
        Rect {
            left: self.left.min(other.left),
            top: self.top.min(other.top),
            right: self.right.max(other.right),
            bottom: self.bottom.max(other.bottom),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

/// Área útil (sem a barra de tarefas) e fator de escala de um monitor.
#[derive(Debug, Clone, Copy)]
pub struct MonitorInfo {
    pub work: Rect,
    pub scale: f64,
}

/// Texto selecionado e sua posição na tela.
#[derive(Debug, Clone)]
pub struct SelectionInfo {
    pub text: String,
    /// União dos retângulos da seleção.
    pub bounds: Rect,
    /// Retângulo da última linha (onde o botão é ancorado).
    pub end_rect: Rect,
    pub truncated: bool,
}

#[derive(Debug, Clone)]
pub enum CaptureEvent {
    Selected(SelectionInfo),
    OutsideClick,
}
