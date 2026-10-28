import type { SettingsErrors } from "@/utils/settingsValidation";

export interface ISectionProps<T> {
  value: T;
  errors: SettingsErrors;
  /** Prefixo das chaves de erro (ex.: "appearance"). */
  prefix: string;
  onChange(patch: Partial<T>): void;
}
