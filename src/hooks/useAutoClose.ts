import { useEffect } from "react";

/** Executa `onClose` após `seconds`, reiniciando a contagem sempre que `resetKey` muda. */
export function useAutoClose(
  enabled: boolean,
  seconds: number,
  paused: boolean,
  resetKey: unknown,
  onClose: () => void,
): void {
  useEffect(() => {
    if (!enabled || paused) return undefined;
    const id = window.setTimeout(onClose, seconds * 1000);
    return () => window.clearTimeout(id);
  }, [enabled, seconds, paused, resetKey, onClose]);
}
