import { Button } from "@/components/ui/Button";
import { IconButton } from "@/components/ui/IconButton";
import { IconX } from "@/components/ui/Icons";
import { LANGUAGES } from "@/constants/languages";
import { LIMITS } from "@/constants/defaultSettings";
import styles from "./TargetLanguagesField.module.css";

interface ITargetLanguagesFieldProps {
  value: string[];
  error?: string;
  onChange(value: string[]): void;
}

const MAX = LIMITS.targetLanguages[1];

/** Lista ordenada de idiomas de destino: o primeiro é o principal; os seguintes servem de fallback. */
export function TargetLanguagesField({ value, error, onChange }: ITargetLanguagesFieldProps): React.JSX.Element {
  const setAt = (index: number, code: string): void => onChange(value.map((c, i) => (i === index ? code : c)));
  const remove = (index: number): void => onChange(value.filter((_, i) => i !== index));
  const move = (index: number, delta: -1 | 1): void => {
    const next = [...value];
    const target = index + delta;
    const current = next[index];
    const other = next[target];
    if (current === undefined || other === undefined) return;
    next[index] = other;
    next[target] = current;
    onChange(next);
  };
  const add = (): void => {
    const free = LANGUAGES.find((l) => !value.some((c) => c.toLowerCase() === l.code.toLowerCase()));
    if (free) onChange([...value, free.code]);
  };

  return (
    <div className={styles.card}>
      <div className={styles.head}>
        <span className={styles.title}>Idiomas de destino</span>
        <span className={styles.description}>
          Em ordem de prioridade. Se o texto já estiver no primeiro idioma, ele é traduzido para o próximo da lista.
        </span>
        {error ? <span className={styles.error}>{error}</span> : null}
      </div>
      <ol className={styles.list}>
        {value.map((code, index) => (
          <li key={`${index}-${code}`} className={styles.item}>
            <span className={styles.rank}>{index + 1}</span>
            <select
              className={styles.select}
              aria-label={`Idioma de destino ${index + 1}`}
              value={code}
              onChange={(event) => setAt(index, event.target.value)}
            >
              {LANGUAGES.map((l) => (
                <option key={l.code} value={l.code}>
                  {l.name}
                </option>
              ))}
            </select>
            <IconButton title="Subir prioridade" disabled={index === 0} onClick={() => move(index, -1)}>
              ↑
            </IconButton>
            <IconButton title="Descer prioridade" disabled={index === value.length - 1} onClick={() => move(index, 1)}>
              ↓
            </IconButton>
            <IconButton title="Remover" disabled={value.length <= 1} onClick={() => remove(index)}>
              <IconX />
            </IconButton>
          </li>
        ))}
      </ol>
      <div className={styles.footer}>
        <Button onClick={add} disabled={value.length >= MAX}>
          Adicionar idioma
        </Button>
      </div>
    </div>
  );
}
