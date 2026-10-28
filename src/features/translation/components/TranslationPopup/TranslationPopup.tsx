import { useState } from "react";
import type { Ref } from "react";
import { Spinner } from "@/components/feedback/Spinner";
import { ResizeHandles } from "@/components/layout/ResizeHandles";
import { TitleBar } from "@/components/layout/TitleBar";
import { IconButton } from "@/components/ui/IconButton";
import { IconCheck, IconCopy, IconRefresh, IconTranslate } from "@/components/ui/Icons";
import { getLanguageName } from "@/constants/languages";
import type { TranslationStatus } from "@/features/translation/hooks/useTranslation";
import type { ResizeEdge } from "@/types/window";
import styles from "./TranslationPopup.module.css";

export interface ITranslationPopupProps {
  status: TranslationStatus;
  originalText: string;
  translatedText?: string;
  sourceLanguage?: string;
  targetLanguage?: string;
  errorMessage?: string;
  truncated?: boolean;
  copied?: boolean;
  /** Recolhido: mostra apenas a barra de título. */
  collapsed?: boolean;
  /** Controle externo do "original" (a cópia de medição precisa espelhar o pop-up visível). */
  showOriginal?: boolean;
  onToggleOriginal?(): void;
  /** Preenche a janela (após redimensionar/maximizar) em vez de ajustar-se ao conteúdo. */
  fill?: boolean;
  containerRef?: Ref<HTMLDivElement>;
  onCopy?(): void;
  onRetry?(): void;
  onClose?(): void;
  onMinimize?(): void;
  onDragStart?(): void;
  onResizeStart?(edge: ResizeEdge): void;
}

/** Visual do pop-up (puro): usado na janela flutuante real e na pré-visualização das configurações. */
export function TranslationPopup({
  status,
  originalText,
  translatedText,
  sourceLanguage,
  targetLanguage,
  errorMessage,
  truncated = false,
  copied = false,
  collapsed = false,
  showOriginal: showOriginalProp,
  onToggleOriginal,
  fill = false,
  containerRef,
  onCopy,
  onRetry,
  onClose,
  onMinimize,
  onDragStart,
  onResizeStart,
}: ITranslationPopupProps): React.JSX.Element {
  const [internalShowOriginal, setInternalShowOriginal] = useState(false);
  const showOriginal = showOriginalProp ?? internalShowOriginal;
  const toggleOriginal = onToggleOriginal ?? ((): void => setInternalShowOriginal((v) => !v));
  const languages = [getLanguageName(sourceLanguage), getLanguageName(targetLanguage)].filter(Boolean).join(" → ");

  return (
    <div
      className={fill ? `${styles.popup} ${styles.fill}` : styles.popup}
      ref={containerRef}
      role="dialog"
      aria-label="Tradução"
    >
      <TitleBar
        compact
        icon={<IconTranslate size={14} />}
        title={languages || "Tradução"}
        showMaximize={false}
        minimizeTitle={collapsed ? "Expandir" : "Recolher"}
        onMinimize={onMinimize}
        onClose={onClose}
        onDragStart={onDragStart}
      />

      {collapsed ? null : (
        <>
          <div className={styles.body}>
            {status === "loading" ? (
              <div className={styles.loading}>
                <Spinner /> Traduzindo…
              </div>
            ) : null}
            {status === "error" ? <p className={styles.error}>{errorMessage ?? "Não foi possível traduzir."}</p> : null}
            {status === "success" ? <p className={styles.translated}>{translatedText}</p> : null}
            {truncated ? <p className={styles.note}>O texto selecionado foi truncado no limite configurado.</p> : null}
            {showOriginal ? <p className={styles.original}>{originalText}</p> : null}
          </div>
          <footer className={styles.footer}>
            <button type="button" className={styles.link} onClick={toggleOriginal}>
              {showOriginal ? "Ocultar original" : "Mostrar original"}
            </button>
            <div className={styles.footerActions}>
              <IconButton title="Traduzir novamente" onClick={onRetry}>
                <IconRefresh />
              </IconButton>
              <IconButton title="Copiar tradução" disabled={status !== "success"} onClick={onCopy}>
                {copied ? <IconCheck /> : <IconCopy />}
              </IconButton>
            </div>
          </footer>
        </>
      )}

      {onResizeStart && !collapsed ? <ResizeHandles onResizeStart={onResizeStart} /> : null}
    </div>
  );
}
