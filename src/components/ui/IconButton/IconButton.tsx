import type { ReactNode } from "react";
import styles from "./IconButton.module.css";

interface IIconButtonProps {
  title: string;
  disabled?: boolean;
  onClick?(): void;
  children: ReactNode;
}

export function IconButton({ title, disabled = false, onClick, children }: IIconButtonProps): React.JSX.Element {
  return (
    <button type="button" className={styles.button} title={title} aria-label={title} disabled={disabled} onClick={onClick}>
      {children}
    </button>
  );
}
