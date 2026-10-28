import { useServiceStatus } from "@/features/translation/hooks/useServiceStatus";
import { describeStatus } from "@/features/translation/utils/describeStatus";
import { useSettings } from "@/hooks/useSettings";
import styles from "./CaptureTip.module.css";

/** Explica o uso por seleção e mostra o estado do serviço. */
export function CaptureTip(): React.JSX.Element {
  const { settings } = useSettings();
  const status = describeStatus(useServiceStatus());
  const shortcut = settings.behavior.shortcuts.translateSelection;
  return (
    <aside className={styles.tip}>
      <span className={`${styles.chip} ${styles[status.tone] ?? ""}`}>
        <span className={styles.dot} />
        {status.label}
      </span>
      <p className={styles.text}>
        Selecione um texto em qualquer aplicativo e clique no botão que aparece.
        {shortcut ? (
          <>
            {" "}
            Atalho: <kbd className={styles.kbd}>{shortcut}</kbd>
          </>
        ) : null}
      </p>
    </aside>
  );
}
