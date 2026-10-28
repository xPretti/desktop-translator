import { useEffect } from "react";
import { systemService } from "@/features/translation/services/systemService";
import { useSettings } from "@/hooks/useSettings";
import { useTheme } from "@/hooks/useTheme";
import styles from "./FloatingButtonApp.module.css";

/** Botão flutuante real. O texto selecionado nunca passa por aqui: o backend o mantém em memória. */
export function FloatingButtonApp(): React.JSX.Element {
  const { settings } = useSettings();
  useTheme(settings.appearance.theme);

  useEffect(() => {
    document.body.dataset.window = "floating-button";
  }, []);

  return (
    <div className={styles.stage}>
      <button
        type="button"
        className={styles.button}
        title="Traduzir seleção"
        aria-label="Traduzir seleção"
        onMouseDown={(event) => event.preventDefault()}
        onClick={() => void systemService.openTranslationPopup()}
      >
        <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
          <path d="m5 8 6 6" />
          <path d="m4 14 6-6 2-3" />
          <path d="M2 5h12" />
          <path d="M7 2h1" />
          <path d="m22 22-5-10-5 10" />
          <path d="M14 18h6" />
        </svg>
      </button>
    </div>
  );
}
