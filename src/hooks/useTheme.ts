import { useEffect } from "react";
import { resolveTheme } from "@/utils/cssVariables";
import type { ThemeMode } from "@/types/settings";

/** Aplica `data-theme` no documento, acompanhando o tema do sistema no modo automático. */
export function useTheme(theme: ThemeMode): void {
  useEffect(() => {
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const apply = (): void => {
      document.documentElement.dataset.theme = resolveTheme(theme, media.matches);
    };
    apply();
    media.addEventListener("change", apply);
    return () => media.removeEventListener("change", apply);
  }, [theme]);
}
