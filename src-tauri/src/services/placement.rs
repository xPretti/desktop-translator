//! Cálculo puro de posicionamento (sem dependência de janelas), todo em pixels físicos.
//!
//! O UI Automation e o Tauri usam coordenadas físicas de tela quando o processo é
//! "per-monitor DPI aware" (o caso de apps Tauri), portanto nenhuma conversão adicional
//! é aplicada aqui; tamanhos lógicos do CSS são convertidos com o fator de escala do monitor.

use crate::models::geometry::{Rect, Size};
use crate::models::settings::{PlacementFallback, PlacementSide, Positioning};

#[derive(Debug, Clone, Copy)]
pub struct PlacementPrefs {
    pub side: PlacementSide,
    pub offset_x: i32,
    pub offset_y: i32,
    pub distance: i32,
    pub margin: i32,
    pub fallback: PlacementFallback,
}

impl PlacementPrefs {
    pub fn popup(p: &Positioning) -> Self {
        Self {
            side: p.popup_side,
            offset_x: p.offset_x,
            offset_y: p.offset_y,
            distance: p.min_distance,
            margin: p.screen_margin,
            fallback: p.fallback,
        }
    }

    pub fn button(p: &Positioning) -> Self {
        Self {
            side: p.button_side,
            offset_x: p.offset_x,
            offset_y: p.offset_y,
            distance: p.button_distance,
            margin: p.screen_margin,
            fallback: p.fallback,
        }
    }
}

/// Retorna o canto superior esquerdo da janela, mantendo-a dentro da área útil do monitor.
pub fn place(anchor: Rect, size: Size, work: Rect, prefs: &PlacementPrefs) -> (i32, i32) {
    let min_x = work.left + prefs.margin;
    let max_x = (work.right - prefs.margin - size.width).max(min_x);
    let min_y = work.top + prefs.margin;
    let max_y = (work.bottom - prefs.margin - size.height).max(min_y);

    let x = (anchor.left + prefs.offset_x).clamp(min_x, max_x);

    let below = anchor.bottom + prefs.distance + prefs.offset_y;
    let above = anchor.top - prefs.distance - size.height + prefs.offset_y;
    let fits = |y: i32| y >= min_y && y <= max_y;
    let (first, second) = match prefs.side {
        PlacementSide::Below => (below, above),
        PlacementSide::Above => (above, below),
    };

    let y = if fits(first) {
        first
    } else if prefs.fallback == PlacementFallback::Flip && fits(second) {
        second
    } else {
        first.clamp(min_y, max_y)
    };
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORK: Rect = Rect { left: 0, top: 0, right: 1920, bottom: 1080 };
    const SIZE: Size = Size { width: 100, height: 50 };

    fn prefs(side: PlacementSide, fallback: PlacementFallback) -> PlacementPrefs {
        PlacementPrefs { side, offset_x: 0, offset_y: 0, distance: 8, margin: 10, fallback }
    }

    #[test]
    fn places_below_when_there_is_room() {
        let anchor = Rect { left: 100, top: 100, right: 200, bottom: 120 };
        assert_eq!(place(anchor, SIZE, WORK, &prefs(PlacementSide::Below, PlacementFallback::Flip)), (100, 128));
    }

    #[test]
    fn flips_above_near_the_bottom_edge() {
        let anchor = Rect { left: 100, top: 1040, right: 200, bottom: 1060 };
        assert_eq!(place(anchor, SIZE, WORK, &prefs(PlacementSide::Below, PlacementFallback::Flip)), (100, 982));
    }

    #[test]
    fn clamps_when_fallback_is_clamp() {
        let anchor = Rect { left: 100, top: 1040, right: 200, bottom: 1060 };
        assert_eq!(place(anchor, SIZE, WORK, &prefs(PlacementSide::Below, PlacementFallback::Clamp)), (100, 1020));
    }

    #[test]
    fn prefers_above_when_configured_and_fits() {
        let anchor = Rect { left: 100, top: 500, right: 200, bottom: 520 };
        assert_eq!(place(anchor, SIZE, WORK, &prefs(PlacementSide::Above, PlacementFallback::Flip)), (100, 442));
    }

    #[test]
    fn keeps_window_inside_horizontal_bounds() {
        let anchor = Rect { left: 1900, top: 100, right: 1910, bottom: 120 };
        assert_eq!(place(anchor, SIZE, WORK, &prefs(PlacementSide::Below, PlacementFallback::Flip)).0, 1810);
        let left = Rect { left: -50, top: 100, right: -40, bottom: 120 };
        assert_eq!(place(left, SIZE, WORK, &prefs(PlacementSide::Below, PlacementFallback::Flip)).0, 10);
    }

    #[test]
    fn works_on_secondary_monitor_with_negative_origin() {
        let work = Rect { left: -1920, top: 0, right: 0, bottom: 1080 };
        let anchor = Rect { left: -300, top: 100, right: -200, bottom: 120 };
        let (x, y) = place(anchor, SIZE, work, &prefs(PlacementSide::Below, PlacementFallback::Flip));
        assert_eq!((x, y), (-300, 128));
    }

    #[test]
    fn applies_offsets() {
        let anchor = Rect { left: 100, top: 100, right: 200, bottom: 120 };
        let mut p = prefs(PlacementSide::Below, PlacementFallback::Flip);
        p.offset_x = 15;
        p.offset_y = 5;
        assert_eq!(place(anchor, SIZE, WORK, &p), (115, 133));
    }
}
