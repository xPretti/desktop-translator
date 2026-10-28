import type { IAppError } from "@/types/translation";

export function toAppError(error: unknown): IAppError {
  if (typeof error === "object" && error !== null) {
    const candidate = error as Partial<IAppError>;
    if (typeof candidate.code === "string" && typeof candidate.message === "string") {
      return { code: candidate.code, message: candidate.message };
    }
  }
  if (typeof error === "string") return { code: "internal", message: error };
  if (error instanceof Error) return { code: "internal", message: error.message };
  return { code: "internal", message: "Ocorreu um erro inesperado." };
}
