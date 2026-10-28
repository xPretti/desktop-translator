import { useCallback, useRef, useState } from "react";
import { translationService } from "@/features/translation/services/translationService";
import type { IAppError, ITranslationRequest, ITranslationResult, ITranslationService } from "@/types/translation";
import { toAppError } from "@/utils/errors";

export type TranslationStatus = "idle" | "loading" | "success" | "error";

export interface IUseTranslation {
  status: TranslationStatus;
  result: ITranslationResult | null;
  error: IAppError | null;
  translate(request: ITranslationRequest): Promise<ITranslationResult | null>;
  reset(): void;
}

function requestKey(r: ITranslationRequest): string {
  return `${r.sourceLanguage}|${r.targetLanguages.join(",")}|${r.text}`;
}

/** Orquestra uma tradução: estados, prevenção de duplicatas em andamento e descarte de respostas obsoletas. */
export function useTranslation(service: ITranslationService = translationService): IUseTranslation {
  const [status, setStatus] = useState<TranslationStatus>("idle");
  const [result, setResult] = useState<ITranslationResult | null>(null);
  const [error, setError] = useState<IAppError | null>(null);
  const inFlight = useRef<string | null>(null);
  const generation = useRef(0);

  const translate = useCallback(
    async (request: ITranslationRequest): Promise<ITranslationResult | null> => {
      const text = request.text.trim();
      if (!text) {
        setStatus("error");
        setError({ code: "empty_text", message: "Digite ou selecione um texto para traduzir." });
        return null;
      }
      const key = requestKey({ ...request, text });
      if (inFlight.current === key) return null;
      inFlight.current = key;
      const current = ++generation.current;
      setStatus("loading");
      setError(null);
      try {
        const value = await service.translate({ ...request, text });
        if (current !== generation.current) return null;
        setResult(value);
        setStatus("success");
        return value;
      } catch (caught) {
        if (current !== generation.current) return null;
        setError(toAppError(caught));
        setStatus("error");
        return null;
      } finally {
        if (inFlight.current === key) inFlight.current = null;
      }
    },
    [service],
  );

  const reset = useCallback(() => {
    generation.current += 1;
    inFlight.current = null;
    setStatus("idle");
    setResult(null);
    setError(null);
  }, []);

  return { status, result, error, translate, reset };
}
