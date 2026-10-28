import { useEffect } from "react";
import type { ReactNode } from "react";
import { IconButton } from "@/components/ui/IconButton";
import { IconX } from "@/components/ui/Icons";
import styles from "./Modal.module.css";

interface IModalProps {
  title: string;
  onClose(): void;
  children: ReactNode;
  footer?: ReactNode;
}

/** Diálogo modal: fecha com Esc ou clicando no fundo. */
export function Modal({ title, onClose, children, footer }: IModalProps): React.JSX.Element {
  useEffect(() => {
    const onKey = (event: KeyboardEvent): void => {
      if (event.key === "Escape") onClose();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [onClose]);

  return (
    <div
      className={styles.backdrop}
      onMouseDown={(event) => {
        if (event.target === event.currentTarget) onClose();
      }}
    >
      <div className={styles.dialog} role="dialog" aria-modal="true" aria-label={title}>
        <header className={styles.header}>
          <h2 className={styles.title}>{title}</h2>
          <IconButton title="Fechar" onClick={onClose}>
            <IconX />
          </IconButton>
        </header>
        <div className={styles.content}>{children}</div>
        {footer ? <footer className={styles.footer}>{footer}</footer> : null}
      </div>
    </div>
  );
}
