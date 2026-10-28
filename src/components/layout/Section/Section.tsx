import type { ReactNode } from "react";
import styles from "./Section.module.css";

interface ISectionProps {
  title: string;
  description?: string;
  children: ReactNode;
}

export function Section({ title, description, children }: ISectionProps): React.JSX.Element {
  return (
    <section className={styles.section}>
      <header className={styles.header}>
        <h3 className={styles.title}>{title}</h3>
        {description ? <p className={styles.description}>{description}</p> : null}
      </header>
      <div className={styles.grid}>{children}</div>
    </section>
  );
}
