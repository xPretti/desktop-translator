import type { IAppearanceSettings } from "@/types/settings";

export type CssVariableMap = Record<string, string | null>;

/** Converte as preferências de aparência em variáveis CSS (null = remover e usar o tema). */
export function buildPopupVariables(a: IAppearanceSettings): CssVariableMap {
  return {
    "--popup-max-width": `${a.maxWidth}px`,
    "--popup-max-height": `${a.maxHeight}px`,
    "--popup-min-width": `${a.minWidth}px`,
    "--popup-border-width": `${a.borderWidth}px`,
    "--popup-font-size": `${a.fontSize}px`,
    "--popup-padding": `${a.padding}px`,
    "--popup-radius": `${a.borderRadius}px`,
    "--popup-opacity": String(a.opacity),
    "--popup-border-color": a.borderColor || null,
    "--popup-bg": a.backgroundColor || null,
    "--popup-fg": a.textColor || null,
    "--popup-font-family": a.fontFamily || null,
  };
}

export function applyCssVariables(target: HTMLElement, variables: CssVariableMap): void {
  for (const [name, value] of Object.entries(variables)) {
    if (value === null) target.style.removeProperty(name);
    else target.style.setProperty(name, value);
  }
}

export function resolveTheme(
  theme: IAppearanceSettings["theme"],
  prefersDark: boolean,
): "light" | "dark" {
  if (theme === "auto") return prefersDark ? "dark" : "light";
  return theme;
}
