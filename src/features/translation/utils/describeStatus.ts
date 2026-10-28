import type { IServiceStatus } from "@/types/settings";

export type StatusTone = "ok" | "warn" | "off";

export function describeStatus(status: IServiceStatus | null): { tone: StatusTone; label: string } {
  if (!status) return { tone: "off", label: "Verificando…" };
  if (!status.translationEnabled) return { tone: "off", label: "Tradução desativada" };
  if (!status.captureSupported) return { tone: "warn", label: "Sem captura global" };
  return { tone: "ok", label: status.captureEnabled ? "Captura ativa" : "Captura pausada" };
}
