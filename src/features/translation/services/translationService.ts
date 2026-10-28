import { callCommand } from "@/services/tauri";
import type { ITranslationRequest, ITranslationResult, ITranslationService } from "@/types/translation";

/**
 * Implementação concreta: delega ao backend Rust, que mantém credenciais fora do frontend
 * e escolhe o provedor configurado. Para outro provedor, basta outra implementação do contrato.
 */
export class TauriTranslationService implements ITranslationService {
  translate(request: ITranslationRequest): Promise<ITranslationResult> {
    return callCommand<ITranslationResult>("translate", { request });
  }
}

export const translationService: ITranslationService = new TauriTranslationService();
