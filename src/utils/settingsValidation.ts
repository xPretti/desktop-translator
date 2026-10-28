import { LIMITS } from "@/constants/defaultSettings";
import type { IAppSettings } from "@/types/settings";

export type SettingsErrors = Record<string, string>;

const COLOR_PATTERN = /^#([0-9a-f]{3}|[0-9a-f]{6}|[0-9a-f]{8})$/i;
const LANGUAGE_PATTERN = /^[A-Za-z]{2,3}(-[A-Za-z0-9]{2,4})?$/;
const MODS = "(?:(?:Ctrl|Control|Alt|Shift|Super|Win)\\+)+";
const SHORTCUT_PATTERN = new RegExp(`^${MODS}(?:[A-Za-z0-9]|F(?:[1-9]|1[0-9]|2[0-4]))$`, "i");

function range(
  errors: SettingsErrors,
  key: string,
  label: string,
  value: number,
  [min, max]: readonly [number, number],
): void {
  if (!Number.isFinite(value) || value < min || value > max) {
    errors[key] = `${label}: informe um valor entre ${min} e ${max}.`;
  }
}

export function isValidColor(value: string): boolean {
  return value === "" || COLOR_PATTERN.test(value);
}

export function isValidShortcut(value: string): boolean {
  return value.trim() === "" || SHORTCUT_PATTERN.test(value.trim());
}

export function validateSettings(settings: IAppSettings): SettingsErrors {
  const errors: SettingsErrors = {};
  const a = settings.appearance;
  const p = settings.positioning;
  const t = settings.translation;
  const b = settings.behavior;

  range(errors, "appearance.maxWidth", "Largura máxima", a.maxWidth, LIMITS.maxWidth);
  range(errors, "appearance.maxHeight", "Altura máxima", a.maxHeight, LIMITS.maxHeight);
  range(errors, "appearance.minWidth", "Largura mínima", a.minWidth, LIMITS.minWidth);
  if (a.minWidth > a.maxWidth) {
    errors["appearance.minWidth"] = "A largura mínima não pode exceder a máxima.";
  }
  range(errors, "appearance.borderWidth", "Espessura da borda", a.borderWidth, LIMITS.borderWidth);
  range(errors, "appearance.fontSize", "Tamanho da fonte", a.fontSize, LIMITS.fontSize);
  range(errors, "appearance.padding", "Espaçamento interno", a.padding, LIMITS.padding);
  range(errors, "appearance.borderRadius", "Raio das bordas", a.borderRadius, LIMITS.borderRadius);
  range(errors, "appearance.opacity", "Opacidade", a.opacity, LIMITS.opacity);
  for (const key of ["borderColor", "backgroundColor", "textColor"] as const) {
    if (!isValidColor(a[key])) errors[`appearance.${key}`] = "Use uma cor hexadecimal (ex.: #1a73e8).";
  }

  range(errors, "positioning.offsetX", "Deslocamento X", p.offsetX, LIMITS.offset);
  range(errors, "positioning.offsetY", "Deslocamento Y", p.offsetY, LIMITS.offset);
  range(errors, "positioning.minDistance", "Distância mínima", p.minDistance, LIMITS.distance);
  range(errors, "positioning.buttonDistance", "Distância do botão", p.buttonDistance, LIMITS.distance);
  range(errors, "positioning.screenMargin", "Margem da tela", p.screenMargin, LIMITS.screenMargin);

  if (t.sourceLanguage !== "auto" && !LANGUAGE_PATTERN.test(t.sourceLanguage)) {
    errors["translation.sourceLanguage"] = "Idioma de origem inválido.";
  }
  const targets = t.targetLanguages;
  if (targets.length < LIMITS.targetLanguages[0] || targets.length > LIMITS.targetLanguages[1]) {
    errors["translation.targetLanguages"] = `Configure de ${LIMITS.targetLanguages[0]} a ${LIMITS.targetLanguages[1]} idiomas de destino.`;
  } else if (targets.some((code) => !LANGUAGE_PATTERN.test(code))) {
    errors["translation.targetLanguages"] = "Há um idioma de destino inválido.";
  } else if (new Set(targets.map((code) => code.toLowerCase())).size !== targets.length) {
    errors["translation.targetLanguages"] = "Não repita idiomas de destino.";
  }
  range(errors, "translation.maxTextLength", "Comprimento máximo", t.maxTextLength, LIMITS.maxTextLength);
  range(errors, "translation.timeoutSeconds", "Timeout", t.timeoutSeconds, LIMITS.timeoutSeconds);

  range(errors, "behavior.popupAutoCloseSeconds", "Tempo de fechamento", b.popupAutoCloseSeconds, LIMITS.autoCloseSeconds);
  range(errors, "behavior.buttonAutoHideSeconds", "Tempo do botão", b.buttonAutoHideSeconds, LIMITS.buttonAutoHideSeconds);
  for (const key of ["toggleCapture", "openMain", "translateSelection"] as const) {
    if (!isValidShortcut(b.shortcuts[key])) {
      errors[`behavior.shortcuts.${key}`] = "Atalho inválido. Exemplo: Ctrl+Alt+T.";
    }
  }
  return errors;
}

export function hasErrors(errors: SettingsErrors): boolean {
  return Object.keys(errors).length > 0;
}
