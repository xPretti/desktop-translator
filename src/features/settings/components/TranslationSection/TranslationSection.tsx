import { Section } from "@/components/layout/Section";
import { NumberField } from "@/components/ui/Field";
import { Select } from "@/components/ui/Select";
import type { ISelectOption } from "@/components/ui/Select";
import { Switch } from "@/components/ui/Switch";
import { AUTO_LANGUAGE, LANGUAGES } from "@/constants/languages";
import { TargetLanguagesField } from "@/features/settings/components/TargetLanguagesField";
import type { ISectionProps } from "@/features/settings/types/sectionProps";
import type { AfterTranslation, ITranslationSettings } from "@/types/settings";

const SOURCES: ISelectOption[] = [AUTO_LANGUAGE, ...LANGUAGES].map((l) => ({ value: l.code, label: l.name }));
const AFTER = [
  { value: "none", label: "Apenas exibir" },
  { value: "copy", label: "Copiar a tradução automaticamente" },
];

export function TranslationSection({ value, errors, prefix, onChange }: ISectionProps<ITranslationSettings>): React.JSX.Element {
  return (
    <Section title="Tradução" description="O texto é enviado ao Google Tradutor para ser traduzido. Veja a seção de privacidade do README.">
      <Select label="Idioma de origem" options={SOURCES} value={value.sourceLanguage} onValueChange={(v) => onChange({ sourceLanguage: v })} />
      <TargetLanguagesField value={value.targetLanguages} error={errors[`${prefix}.targetLanguages`]} onChange={(v) => onChange({ targetLanguages: v })} />
      <Select label="Após traduzir" options={AFTER} value={value.afterTranslation} onValueChange={(v) => onChange({ afterTranslation: v as AfterTranslation })} />
      <NumberField label="Comprimento máximo (caracteres)" value={value.maxTextLength} error={errors[`${prefix}.maxTextLength`]} onValueChange={(v) => onChange({ maxTextLength: v })} />
      <NumberField label="Timeout (segundos)" value={value.timeoutSeconds} error={errors[`${prefix}.timeoutSeconds`]} onValueChange={(v) => onChange({ timeoutSeconds: v })} />
      <Switch label="Serviço de tradução ativo" checked={value.translationEnabled} onCheckedChange={(v) => onChange({ translationEnabled: v })} />
      <Switch label="Captura global de seleção" description="Detecta textos selecionados em outros aplicativos." checked={value.captureEnabled} onCheckedChange={(v) => onChange({ captureEnabled: v })} />
      <Switch label="Capturar texto de navegadores e outros apps (via cópia)" description="Quando a leitura direta falha, copia a seleção com Ctrl+C e restaura o clipboard em seguida." checked={value.clipboardFallback} onCheckedChange={(v) => onChange({ clipboardFallback: v })} />
      <Switch label="Traduzir automaticamente após selecionar" description="Abre o pop-up sem clicar no botão." checked={value.autoTranslate} onCheckedChange={(v) => onChange({ autoTranslate: v })} />
      <Switch label="Cache de traduções recentes" checked={value.cacheEnabled} onCheckedChange={(v) => onChange({ cacheEnabled: v })} />
    </Section>
  );
}
