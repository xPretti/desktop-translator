import { invoke } from "@tauri-apps/api/core";
import { toAppError } from "@/utils/errors";

/** Único ponto de acesso a `invoke`: padroniza o tratamento de erros dos comandos Rust. */
export async function callCommand<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw toAppError(error);
  }
}
