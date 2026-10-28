export interface ITranslationRequest {
  text: string;
  sourceLanguage: string;
  /** Destinos em ordem de prioridade; o backend usa o primeiro diferente do idioma do texto. */
  targetLanguages: string[];
}

export interface ITranslationResult {
  originalText: string;
  translatedText: string;
  detectedLanguage?: string;
  /** Destino efetivamente usado. */
  targetLanguage: string;
}

export interface ITranslationService {
  translate(request: ITranslationRequest): Promise<ITranslationResult>;
}

export interface IPopupShowPayload {
  text: string;
  truncated: boolean;
  requestId: number;
  /** Preenchido quando a captura falhou (ex.: nenhuma seleção). */
  error?: string;
}

export interface IAppError {
  code: string;
  message: string;
}
