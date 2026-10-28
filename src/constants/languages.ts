export interface ILanguage {
  code: string;
  name: string;
}

export const LANGUAGES: readonly ILanguage[] = [
  { code: "ar", name: "Árabe" },
  { code: "bg", name: "Búlgaro" },
  { code: "zh-CN", name: "Chinês (simplificado)" },
  { code: "zh-TW", name: "Chinês (tradicional)" },
  { code: "ko", name: "Coreano" },
  { code: "cs", name: "Tcheco" },
  { code: "da", name: "Dinamarquês" },
  { code: "es", name: "Espanhol" },
  { code: "fi", name: "Finlandês" },
  { code: "fr", name: "Francês" },
  { code: "el", name: "Grego" },
  { code: "he", name: "Hebraico" },
  { code: "hi", name: "Hindi" },
  { code: "nl", name: "Holandês" },
  { code: "hu", name: "Húngaro" },
  { code: "id", name: "Indonésio" },
  { code: "en", name: "Inglês" },
  { code: "it", name: "Italiano" },
  { code: "ja", name: "Japonês" },
  { code: "no", name: "Norueguês" },
  { code: "pl", name: "Polonês" },
  { code: "pt", name: "Português" },
  { code: "ro", name: "Romeno" },
  { code: "ru", name: "Russo" },
  { code: "sv", name: "Sueco" },
  { code: "th", name: "Tailandês" },
  { code: "tr", name: "Turco" },
  { code: "uk", name: "Ucraniano" },
  { code: "vi", name: "Vietnamita" },
];

export const AUTO_LANGUAGE: ILanguage = { code: "auto", name: "Detectar idioma" };

export function getLanguageName(code: string | undefined): string {
  if (!code) return "";
  if (code === "auto") return AUTO_LANGUAGE.name;
  const exact = LANGUAGES.find((l) => l.code.toLowerCase() === code.toLowerCase());
  if (exact) return exact.name;
  const base = code.split("-")[0]?.toLowerCase();
  return LANGUAGES.find((l) => l.code.toLowerCase() === base)?.name ?? code;
}
