import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useRef, useState } from "react";
import { TranslationPopup } from "@/features/translation/components/TranslationPopup";
import { useTranslation } from "@/features/translation/hooks/useTranslation";
import { systemService } from "@/features/translation/services/systemService";
import { useAutoClose } from "@/hooks/useAutoClose";
import { useCopy } from "@/hooks/useCopy";
import { usePopupVariables } from "@/hooks/usePopupVariables";
import { useSettings } from "@/hooks/useSettings";
import { useTheme } from "@/hooks/useTheme";
import { useWindowControls } from "@/hooks/useWindowControls";
import type { IPopupShowPayload } from "@/types/translation";
import type { ResizeEdge } from "@/types/window";
import styles from "./PopupApp.module.css";

/**
 * Janela flutuante real. O pop-up visível sempre preenche a janela (sem alternar de layout ao
 * redimensionar); o tamanho "natural" do conteúdo é medido em uma cópia invisível e enviado ao
 * backend, que ajusta a janela enquanto o usuário não a redimensiona manualmente.
 */
export function PopupApp(): React.JSX.Element {
  const { settings } = useSettings();
  const { status, result, error, translate, reset } = useTranslation();
  const { copied, copy } = useCopy();
  const controls = useWindowControls("popup");
  const [payload, setPayload] = useState<IPopupShowPayload | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [hovered, setHovered] = useState(false);
  const [userSized, setUserSized] = useState(false);
  const [collapsed, setCollapsed] = useState(false);
  const [showOriginal, setShowOriginal] = useState(false);
  const measureRef = useRef<HTMLDivElement>(null);
  const settingsRef = useRef(settings);
  const userSizedRef = useRef(false);
  const lastSizeRef = useRef("");
  settingsRef.current = settings;
  userSizedRef.current = userSized;

  useTheme(settings.appearance.theme);
  usePopupVariables("root", settings.appearance);

  const run = useCallback(
    async (text: string) => {
      const { sourceLanguage, targetLanguages, afterTranslation } = settingsRef.current.translation;
      const value = await translate({ text, sourceLanguage, targetLanguages });
      if (value && afterTranslation === "copy") await copy(value.translatedText);
    },
    [translate, copy],
  );

  useEffect(() => {
    document.body.dataset.window = "popup";
    let unlisten: (() => void) | undefined;
    let active = true;
    listen<IPopupShowPayload>("popup-show", (event) => {
      reset();
      setUserSized(false);
      setCollapsed(false);
      setShowOriginal(false);
      lastSizeRef.current = "";
      setPayload(event.payload);
      setNotice(event.payload.error ?? null);
      if (!event.payload.error) void run(event.payload.text);
    }).then((fn) => {
      if (active) unlisten = fn;
      else fn();
    });
    return () => {
      active = false;
      unlisten?.();
    };
  }, [reset, run]);

  const reportSize = useCallback(() => {
    const element = measureRef.current;
    if (!element || userSizedRef.current) return;
    const rect = element.getBoundingClientRect();
    const key = `${Math.ceil(rect.width)}x${Math.ceil(rect.height)}`;
    if (key === lastSizeRef.current) return;
    lastSizeRef.current = key;
    void systemService.resizePopup(Math.ceil(rect.width), Math.ceil(rect.height));
  }, []);

  useEffect(() => {
    const element = measureRef.current;
    if (!element) return undefined;
    let frame = 0;
    const observer = new ResizeObserver(() => {
      cancelAnimationFrame(frame);
      frame = requestAnimationFrame(reportSize);
    });
    observer.observe(element);
    return () => {
      cancelAnimationFrame(frame);
      observer.disconnect();
    };
  }, [reportSize]);

  // Ao voltar ao modo automático (ex.: recolher), reaplica o tamanho do conteúdo.
  useEffect(() => {
    if (!userSized) {
      lastSizeRef.current = "";
      reportSize();
    }
  }, [userSized, reportSize]);

  const close = useCallback(() => void systemService.hidePopup(), []);
  useAutoClose(
    settings.behavior.popupAutoClose,
    settings.behavior.popupAutoCloseSeconds,
    hovered || status === "loading",
    payload?.requestId ?? status,
    close,
  );

  const beginResize = (edge: ResizeEdge): void => {
    setUserSized(true);
    controls.beginResize(edge);
  };

  const common = {
    status: notice ? ("error" as const) : status,
    originalText: payload?.text ?? "",
    translatedText: result?.translatedText,
    sourceLanguage: result?.detectedLanguage ?? settings.translation.sourceLanguage,
    targetLanguage: result?.targetLanguage ?? settings.translation.targetLanguages[0],
    errorMessage: notice ?? error?.message,
    truncated: payload?.truncated,
    collapsed,
    showOriginal,
  };

  return (
    <>
      <div className={styles.stage} onMouseEnter={() => setHovered(true)} onMouseLeave={() => setHovered(false)}>
        <TranslationPopup
          {...common}
          copied={copied}
          fill
          onToggleOriginal={() => setShowOriginal((value) => !value)}
          onDragStart={controls.beginDrag}
          onResizeStart={beginResize}
          onMinimize={() => {
            setCollapsed((value) => !value);
            setUserSized(false);
          }}
          onCopy={() => result && void copy(result.translatedText)}
          onRetry={() => {
            if (!payload?.text) return;
            setNotice(null);
            void run(payload.text);
          }}
          onClose={close}
        />
      </div>
      {/* Cópia invisível, em layout automático, usada só para medir o tamanho do conteúdo. */}
      <div className={styles.measure} ref={measureRef} aria-hidden="true" inert>
        <TranslationPopup {...common} />
      </div>
    </>
  );
}
