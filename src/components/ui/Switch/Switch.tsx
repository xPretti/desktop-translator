import { useId } from "react";
import { SettingRow } from "@/components/ui/SettingRow";
import styles from "./Switch.module.css";

interface ISwitchProps {
  label: string;
  description?: string;
  checked: boolean;
  disabled?: boolean;
  onCheckedChange(checked: boolean): void;
}

/** Interruptor no estilo do Windows 11, com o estado escrito ao lado. */
export function Switch({ label, description, checked, disabled = false, onCheckedChange }: ISwitchProps): React.JSX.Element {
  const id = useId();
  return (
    <SettingRow title={label} description={description} htmlFor={id}>
      <span className={styles.state}>{checked ? "Ativado" : "Desativado"}</span>
      <input
        id={id}
        className={styles.input}
        type="checkbox"
        role="switch"
        checked={checked}
        disabled={disabled}
        onChange={(event) => onCheckedChange(event.target.checked)}
      />
    </SettingRow>
  );
}
