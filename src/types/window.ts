export type WindowLabel = "main" | "popup";
export type ResizeEdge = "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw";
export type WindowInteraction = "move" | `resize-${ResizeEdge}`;
export type WindowAction = "minimize" | "toggle_maximize" | "close";
