import { listen } from "@tauri-apps/api/event";
import { useEffect, useMemo, useState } from "react";
import { systemService } from "@/features/translation/services/systemService";
import type { ResizeEdge, WindowAction, WindowLabel } from "@/types/window";

export interface IWindowControls {
  maximized: boolean;
  minimize(): void;
  toggleMaximize(): void;
  close(): void;
  beginDrag(): void;
  beginResize(edge: ResizeEdge): void;
}

/** Controles de janela customizados (a janela não tem decoração nativa). */
export function useWindowControls(label: WindowLabel): IWindowControls {
  const [maximized, setMaximized] = useState(false);

  useEffect(() => {
    let active = true;
    let unlisten: (() => void) | undefined;
    listen<boolean>("window-maximized", (event) => setMaximized(event.payload)).then((fn) => {
      if (active) unlisten = fn;
      else fn();
    });
    return () => {
      active = false;
      unlisten?.();
    };
  }, []);

  return useMemo(() => {
    const act = (action: WindowAction): void => {
      systemService.windowControl(label, action).catch(() => undefined);
    };
    return {
      maximized,
      minimize: () => act("minimize"),
      toggleMaximize: () => act("toggle_maximize"),
      close: () => act("close"),
      beginDrag: () => {
        systemService.beginWindowInteraction(label, "move").catch(() => undefined);
      },
      beginResize: (edge) => {
        systemService.beginWindowInteraction(label, `resize-${edge}`).catch(() => undefined);
      },
    };
  }, [label, maximized]);
}
