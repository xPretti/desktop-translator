import { useEffect } from "react";
import type { RefObject } from "react";
import { applyCssVariables, buildPopupVariables } from "@/utils/cssVariables";
import type { IAppearanceSettings } from "@/types/settings";

/** Aplica as preferências visuais como variáveis CSS no elemento alvo (imediato, sem reiniciar). */
export function usePopupVariables(
  target: RefObject<HTMLElement | null> | "root",
  appearance: IAppearanceSettings,
): void {
  useEffect(() => {
    const element = target === "root" ? document.documentElement : target.current;
    if (element) applyCssVariables(element, buildPopupVariables(appearance));
  }, [target, appearance]);
}
