import { useContext } from "react";
import { SettingsContext } from "@/contexts/SettingsContext";
import type { ISettingsContextValue } from "@/contexts/SettingsContext";

export function useSettings(): ISettingsContextValue {
  const context = useContext(SettingsContext);
  if (!context) throw new Error("useSettings deve ser usado dentro de SettingsProvider.");
  return context;
}
