import { useState } from "react";
import { Spinner } from "@/components/feedback/Spinner";
import { Button } from "@/components/ui/Button";
import { IconButton } from "@/components/ui/IconButton";
import { IconCheck, IconCopy, IconSwap } from "@/components/ui/Icons";
import { AUTO_LANGUAGE, getLanguageName, LANGUAGES } from "@/constants/languages";
import { CaptureTip } from "@/features/translation/components/CaptureTip";
import { useTranslation } from "@/features/translation/hooks/useTranslation";
import { useCopy } from "@/hooks/useCopy";
import { useSettings } from "@/hooks/useSettings";
import type { ITranslationSettings } from "@/types/settings";
import styles from "./TranslationPanel.module.css";

/** Página inicial: tradução manual (independente da captura global de seleção). */
export function TranslationPanel(): React.JSX.Element {
  const { settings, save } = useSettings();
  const { status, result, error, translate } = useTranslation();
  const { copied, copy } = useCopy();
  const [text, setText] = useState("");
  const { sourceLanguage, targetLanguages, maxTextLength } = settings.translation;
  const primaryTarget = targetLanguages[0] ?? "pt";

  const run = (): void => void translate({ text, sourceLanguage, targetLanguages });
  const change = (patch: Partial<ITranslationSettings>): void =>
    void save({ ...settings, translation: { ...settings.translation, ...patch } });
  // Escolher um destino o torna o principal; os demais continuam como fallback.
  const setPrimaryTarget = (code: string): void =>
    change({ targetLanguages: [code, ...targetLanguages.filter((c) => c.toLowerCase() !== code.toLowerCase())].slice(0, 5) });
  const swap = (): void => {
    change({ sourceLanguage: primaryTarget, targetLanguages: [sourceLanguage, ...targetLanguages.filter((c) => c.toLowerCase() !== sourceLanguage.toLowerCase())].slice(0, 5) });
  };

  return (
    <div className={styles.panel}>
      <header className={styles.hero}>
        <h1 className={styles.title}>Traduzir</h1>
        <p className={styles.subtitle}>Digite um texto abaixo ou selecione-o em qualquer aplicativo.</p>
      </header>

      <div className={styles.languages}>
        <select
          className={styles.select}
          aria-label="Idioma de origem"
          value={sourceLanguage}
          onChange={(event) => change({ sourceLanguage: event.target.value })}
        >
          {[AUTO_LANGUAGE, ...LANGUAGES].map((l) => (
            <option key={l.code} value={l.code}>
              {l.name}
            </option>
          ))}
        </select>
        <IconButton
          title="Inverter idiomas"
          disabled={sourceLanguage === "auto"}
          onClick={swap}
        >
          <IconSwap />
        </IconButton>
        <select
          className={styles.select}
          aria-label="Idioma de destino"
          value={primaryTarget}
          onChange={(event) => setPrimaryTarget(event.target.value)}
        >
          {LANGUAGES.map((l) => (
            <option key={l.code} value={l.code}>
              {l.name}
            </option>
          ))}
        </select>
      </div>

      <section className={styles.card}>
        <textarea
          className={styles.textarea}
          value={text}
          maxLength={maxTextLength}
          placeholder="Digite ou cole o texto aqui…"
          aria-label="Texto a traduzir"
          onChange={(event) => setText(event.target.value)}
          onKeyDown={(event) => event.ctrlKey && event.key === "Enter" && run()}
        />
        <div className={styles.cardFooter}>
          <span className={styles.counter}>
            {text.length}/{maxTextLength}
          </span>
          <Button variant="primary" onClick={run} disabled={status === "loading" || !text.trim()} title="Ctrl+Enter">
            {status === "loading" ? <Spinner /> : null}
            Traduzir
          </Button>
        </div>
      </section>

      <section className={styles.card} aria-live="polite">
        <div className={styles.resultHead}>
          <span>{getLanguageName(result?.targetLanguage ?? primaryTarget)}</span>
          {result?.detectedLanguage ? <span className={styles.detected}>detectado: {getLanguageName(result.detectedLanguage)}</span> : null}
        </div>
        <div className={styles.resultBody}>
          {status === "idle" ? <p className={styles.muted}>A tradução aparecerá aqui.</p> : null}
          {status === "loading" ? <p className={styles.muted}>Traduzindo…</p> : null}
          {status === "error" ? <p className={styles.error}>{error?.message}</p> : null}
          {status === "success" && result ? <p className={styles.output}>{result.translatedText}</p> : null}
        </div>
        <div className={styles.cardFooter}>
          <span />
          <Button onClick={() => result && void copy(result.translatedText)} disabled={!result} icon={copied ? <IconCheck /> : <IconCopy />}>
            {copied ? "Copiado" : "Copiar"}
          </Button>
        </div>
      </section>

      <CaptureTip />
    </div>
  );
}
