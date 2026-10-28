import styles from "./Spinner.module.css";

export function Spinner({ label = "Carregando" }: { label?: string }): React.JSX.Element {
  return <span className={styles.spinner} role="status" aria-label={label} />;
}
