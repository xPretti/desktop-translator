import type { ReactNode } from "react";
import styles from "./SettingRow.module.css";

interface ISettingRowProps {
  title: string;
  description?: string;
  error?: string;
  htmlFor?: string;
  children: ReactNode;
}

/** Linha de configuração em cartão: texto à esquerda, controle à direita. */
export function SettingRow({ title, description, error, htmlFor, children }: ISettingRowProps): React.JSX.Element {
  return (
    <div className={styles.row}>
      <div className={styles.text}>
        {htmlFor ? (
          <label className={styles.title} htmlFor={htmlFor}>
            {title}
          </label>
        ) : (
          <span className={styles.title}>{title}</span>
        )}
        {description ? <span className={styles.description}>{description}</span> : null}
        {error ? <span className={styles.error}>{error}</span> : null}
      </div>
      <div className={styles.control}>{children}</div>
    </div>
  );
}
