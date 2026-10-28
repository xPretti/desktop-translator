import type { ReactNode } from "react";
import { IconClose, IconMaximize, IconMinimize, IconRestore } from "@/components/ui/Icons";
import styles from "./TitleBar.module.css";

export interface ITitleBarProps {
  icon?: ReactNode;
  title: ReactNode;
  /** Botões extras antes dos controles de janela. */
  actions?: ReactNode;
  maximized?: boolean;
  compact?: boolean;
  showMaximize?: boolean;
  minimizeTitle?: string;
  onMinimize?(): void;
  onToggleMaximize?(): void;
  onClose?(): void;
  onDragStart?(): void;
}

/** Barra de título própria (arrastar, duplo clique, minimizar/maximizar/fechar). */
export function TitleBar({
  icon,
  title,
  actions,
  maximized = false,
  compact = false,
  showMaximize = true,
  minimizeTitle = "Minimizar",
  onMinimize,
  onToggleMaximize,
  onClose,
  onDragStart,
}: ITitleBarProps): React.JSX.Element {
  return (
    <div
      className={compact ? `${styles.bar} ${styles.compact}` : styles.bar}
      onMouseDown={(event) => {
        if (event.button !== 0 || (event.target as HTMLElement).closest("button")) return;
        event.preventDefault();
        if (event.detail >= 2) onToggleMaximize?.();
        else onDragStart?.();
      }}
    >
      <div className={styles.title}>
        {icon}
        <span className={styles.text}>{title}</span>
      </div>
      {actions ? <div className={styles.actions}>{actions}</div> : null}
      <div className={styles.controls}>
        <button type="button" className={styles.control} title={minimizeTitle} aria-label={minimizeTitle} onClick={onMinimize}>
          <IconMinimize />
        </button>
        {showMaximize ? (
          <button
            type="button"
            className={styles.control}
            title={maximized ? "Restaurar" : "Maximizar"}
            aria-label={maximized ? "Restaurar" : "Maximizar"}
            onClick={onToggleMaximize}
          >
            {maximized ? <IconRestore /> : <IconMaximize />}
          </button>
        ) : null}
        <button type="button" className={`${styles.control} ${styles.close}`} title="Fechar" aria-label="Fechar" onClick={onClose}>
          <IconClose />
        </button>
      </div>
    </div>
  );
}
