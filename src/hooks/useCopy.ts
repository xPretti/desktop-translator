import { useCallback, useEffect, useRef, useState } from "react";
import { systemService } from "@/features/translation/services/systemService";

export interface IUseCopy {
  copied: boolean;
  copy(text: string): Promise<void>;
}

/** Copia via backend (funciona mesmo em janelas sem foco) e sinaliza sucesso por instantes. */
export function useCopy(): IUseCopy {
  const [copied, setCopied] = useState(false);
  const timer = useRef<number | undefined>(undefined);

  useEffect(() => () => window.clearTimeout(timer.current), []);

  const copy = useCallback(async (text: string) => {
    if (!text) return;
    await systemService.copyText(text);
    setCopied(true);
    window.clearTimeout(timer.current);
    timer.current = window.setTimeout(() => setCopied(false), 1500);
  }, []);

  return { copied, copy };
}
