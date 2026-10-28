import { useEffect, useMemo, useRef, useState } from "react";
import { Modal } from "@/components/layout/Modal";
import { Button } from "@/components/ui/Button";
import { AppearanceSection } from "@/features/settings/components/AppearanceSection";
import { BehaviorSection } from "@/features/settings/components/BehaviorSection";
import { PopupPreview } from "@/features/settings/components/PopupPreview";
import { PositioningSection } from "@/features/settings/components/PositioningSection";
import { TranslationSection } from "@/features/settings/components/TranslationSection";
import { useSettings } from "@/hooks/useSettings";
import type { IAppSettings } from "@/types/settings";
import { toAppError } from "@/utils/errors";
import { hasErrors, validateSettings } from "@/utils/settingsValidation";
import styles from "./SettingsModal.module.css";

type Category = "translation" | "appearance" | "positioning" | "behavior";
type Feedback = { tone: "ok" | "error"; text: string } | null;

const CATEGORIES: readonly { id: Category; label: string }[] = [
  { id: "translation", label: "Tradução" },
  { id: "appearance", label: "Aparência do pop-up" },
  { id: "positioning", label: "Posicionamento" },
  { id: "behavior", label: "Comportamento e atalhos" },
];

interface ISettingsModalProps {
  onClose(): void;
}

/** Modal de configurações: rascunho local, validação imediata e salvamento explícito. */
export function SettingsModal({ onClose }: ISettingsModalProps): React.JSX.Element {
  const { settings, save, reset } = useSettings();
  const [draft, setDraft] = useState<IAppSettings>(settings);
  const [category, setCategory] = useState<Category>("translation");
  const [feedback, setFeedback] = useState<Feedback>(null);
  const [saving, setSaving] = useState(false);
  const lastSaved = useRef(settings);

  // Mudanças externas (ex.: bandeja) só substituem o rascunho se ele não foi editado.
  useEffect(() => {
    if (JSON.stringify(draft) === JSON.stringify(lastSaved.current)) setDraft(settings);
    lastSaved.current = settings;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [settings]);

  const errors = useMemo(() => validateSettings(draft), [draft]);
  const dirty = JSON.stringify(draft) !== JSON.stringify(settings);
  const invalidCategories = useMemo(
    () => CATEGORIES.filter((c) => Object.keys(errors).some((key) => key.startsWith(`${c.id}.`))).map((c) => c.label),
    [errors],
  );

  const update = <K extends keyof IAppSettings>(group: K, patch: Partial<IAppSettings[K]>): void => {
    setFeedback(null);
    setDraft((current) => ({ ...current, [group]: { ...current[group], ...patch } }));
  };

  const finish = (warnings: string[], okText: string): void =>
    setFeedback(warnings.length ? { tone: "error", text: warnings.join(" ") } : { tone: "ok", text: okText });

  const onSave = async (): Promise<void> => {
    setSaving(true);
    try {
      const result = await save(draft);
      setDraft(result.settings);
      finish(result.warnings, "Configurações salvas.");
    } catch (error) {
      setFeedback({ tone: "error", text: toAppError(error).message });
    } finally {
      setSaving(false);
    }
  };

  const onReset = async (): Promise<void> => {
    try {
      const result = await reset();
      setDraft(result.settings);
      finish(result.warnings, "Valores padrão restaurados.");
    } catch (error) {
      setFeedback({ tone: "error", text: toAppError(error).message });
    }
  };

  const status = feedback
    ? feedback.text
    : hasErrors(errors)
      ? `Corrija os campos em: ${invalidCategories.join(", ")}.`
      : dirty
        ? "Alterações não salvas."
        : "";

  const footer = (
    <div className={styles.footer}>
      <span className={feedback?.tone === "error" || hasErrors(errors) ? styles.error : styles.ok} role="status">
        {status}
      </span>
      <div className={styles.buttons}>
        <Button onClick={() => void onReset()}>Restaurar padrões</Button>
        <Button variant="primary" onClick={() => void onSave()} disabled={!dirty || hasErrors(errors) || saving}>
          Salvar
        </Button>
      </div>
    </div>
  );

  return (
    <Modal title="Configurações" onClose={onClose} footer={footer}>
      <div className={styles.layout}>
        <nav className={styles.nav} aria-label="Categorias de configuração">
          {CATEGORIES.map((item) => (
            <button
              key={item.id}
              type="button"
              className={item.id === category ? `${styles.navItem} ${styles.active}` : styles.navItem}
              onClick={() => setCategory(item.id)}
            >
              {item.label}
            </button>
          ))}
        </nav>
        <div className={styles.scroll}>
          {category === "translation" ? (
            <TranslationSection prefix="translation" value={draft.translation} errors={errors} onChange={(p) => update("translation", p)} />
          ) : null}
          {category === "appearance" ? (
            <>
              <PopupPreview appearance={draft.appearance} targetLanguage={draft.translation.targetLanguages[0]} />
              <AppearanceSection prefix="appearance" value={draft.appearance} errors={errors} onChange={(p) => update("appearance", p)} />
            </>
          ) : null}
          {category === "positioning" ? (
            <PositioningSection prefix="positioning" value={draft.positioning} errors={errors} onChange={(p) => update("positioning", p)} />
          ) : null}
          {category === "behavior" ? (
            <BehaviorSection prefix="behavior" value={draft.behavior} errors={errors} onChange={(p) => update("behavior", p)} />
          ) : null}
        </div>
      </div>
    </Modal>
  );
}
