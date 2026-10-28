import { useRef } from "react";
import { TranslationPopup } from "@/features/translation/components/TranslationPopup";
import { usePopupVariables } from "@/hooks/usePopupVariables";
import type { IAppearanceSettings } from "@/types/settings";
import styles from "./PopupPreview.module.css";

interface IPopupPreviewProps {
  appearance: IAppearanceSettings;
  targetLanguage?: string;
}

/** Pré-visualização em tempo real: usa o mesmo componente e as mesmas variáveis CSS do pop-up real. */
export function PopupPreview({ appearance, targetLanguage }: IPopupPreviewProps): React.JSX.Element {
  const stageRef = useRef<HTMLDivElement>(null);
  usePopupVariables(stageRef, appearance);
  return (
    <div className={styles.stage} ref={stageRef} data-theme={appearance.theme === "auto" ? undefined : appearance.theme}>
      <TranslationPopup
        status="success"
        originalText="The quick brown fox jumps over the lazy dog."
        translatedText="A rápida raposa marrom pula sobre o cão preguiçoso."
        sourceLanguage="en"
        targetLanguage={targetLanguage}
      />
    </div>
  );
}
