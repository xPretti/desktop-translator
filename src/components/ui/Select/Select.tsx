import { useId } from "react";
import type { SelectHTMLAttributes } from "react";
import { SettingRow } from "@/components/ui/SettingRow";
import styles from "./Select.module.css";

export interface ISelectOption {
  value: string;
  label: string;
}

interface ISelectProps extends Omit<SelectHTMLAttributes<HTMLSelectElement>, "onChange"> {
  label: string;
  description?: string;
  options: readonly ISelectOption[];
  value: string;
  onValueChange(value: string): void;
}

export function Select({ label, description, options, value, onValueChange, ...rest }: ISelectProps): React.JSX.Element {
  const id = useId();
  return (
    <SettingRow title={label} description={description} htmlFor={id}>
      <select id={id} className={styles.select} value={value} onChange={(event) => onValueChange(event.target.value)} {...rest}>
        {options.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </SettingRow>
  );
}
