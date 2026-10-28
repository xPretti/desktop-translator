import { useId } from "react";
import { SettingRow } from "@/components/ui/SettingRow";
import styles from "./Field.module.css";

interface IBaseFieldProps {
  label: string;
  error?: string;
  hint?: string;
}

interface ITextFieldProps extends IBaseFieldProps {
  value: string;
  placeholder?: string;
  type?: "text" | "password";
  onValueChange(value: string): void;
}

interface INumberFieldProps extends IBaseFieldProps {
  value: number;
  step?: number;
  onValueChange(value: number): void;
}

interface IColorFieldProps extends IBaseFieldProps {
  value: string;
  onValueChange(value: string): void;
}

export function TextField({ label, hint, error, value, placeholder, type = "text", onValueChange }: ITextFieldProps): React.JSX.Element {
  const id = useId();
  return (
    <SettingRow title={label} description={hint} error={error} htmlFor={id}>
      <input
        id={id}
        className={styles.input}
        type={type}
        value={value}
        placeholder={placeholder}
        aria-invalid={Boolean(error)}
        onChange={(event) => onValueChange(event.target.value)}
      />
    </SettingRow>
  );
}

export function NumberField({ label, hint, error, value, step = 1, onValueChange }: INumberFieldProps): React.JSX.Element {
  const id = useId();
  return (
    <SettingRow title={label} description={hint} error={error} htmlFor={id}>
      <input
        id={id}
        className={styles.input}
        type="number"
        step={step}
        value={Number.isFinite(value) ? value : ""}
        aria-invalid={Boolean(error)}
        onChange={(event) => onValueChange(event.target.valueAsNumber)}
      />
    </SettingRow>
  );
}

/** Cor hexadecimal; vazio = usar o padrão do tema. */
export function ColorField({ label, hint, error, value, onValueChange }: IColorFieldProps): React.JSX.Element {
  const id = useId();
  const swatch = /^#[0-9a-f]{6}$/i.test(value) ? value : "#888888";
  return (
    <SettingRow title={label} description={hint} error={error} htmlFor={id}>
      <input
        className={styles.swatch}
        type="color"
        aria-label={`${label} (seletor)`}
        value={swatch}
        onChange={(event) => onValueChange(event.target.value)}
      />
      <input
        id={id}
        className={styles.input}
        type="text"
        value={value}
        placeholder="Padrão do tema"
        aria-invalid={Boolean(error)}
        onChange={(event) => onValueChange(event.target.value.trim())}
      />
    </SettingRow>
  );
}
