import { Section } from "@/components/layout/Section";
import { ColorField, NumberField, TextField } from "@/components/ui/Field";
import { Select } from "@/components/ui/Select";
import type { ISectionProps } from "@/features/settings/types/sectionProps";
import type { IAppearanceSettings, ThemeMode } from "@/types/settings";

const THEMES = [
  { value: "auto", label: "Automático (sistema)" },
  { value: "light", label: "Claro" },
  { value: "dark", label: "Escuro" },
];

export function AppearanceSection({ value, errors, prefix, onChange }: ISectionProps<IAppearanceSettings>): React.JSX.Element {
  const err = (key: string): string | undefined => errors[`${prefix}.${key}`];
  const num = (key: keyof IAppearanceSettings, label: string, step = 1): React.JSX.Element => (
    <NumberField label={label} step={step} value={value[key] as number} error={err(key)} onValueChange={(v) => onChange({ [key]: v })} />
  );
  return (
    <Section title="Aparência do pop-up" description="As alterações são aplicadas imediatamente na pré-visualização e no pop-up.">
      <Select label="Tema" options={THEMES} value={value.theme} onValueChange={(v) => onChange({ theme: v as ThemeMode })} />
      {num("maxWidth", "Largura máxima (px)")}
      {num("minWidth", "Largura mínima (px)")}
      {num("maxHeight", "Altura máxima (px)")}
      {num("fontSize", "Tamanho da fonte (px)")}
      {num("padding", "Espaçamento interno (px)")}
      {num("borderRadius", "Raio das bordas (px)")}
      {num("borderWidth", "Espessura da borda (px)")}
      {num("opacity", "Opacidade (0,4–1)", 0.05)}
      <ColorField label="Cor de fundo" value={value.backgroundColor} error={err("backgroundColor")} onValueChange={(v) => onChange({ backgroundColor: v })} />
      <ColorField label="Cor do texto" value={value.textColor} error={err("textColor")} onValueChange={(v) => onChange({ textColor: v })} />
      <ColorField label="Cor da borda" value={value.borderColor} error={err("borderColor")} onValueChange={(v) => onChange({ borderColor: v })} />
      <TextField label="Família tipográfica" value={value.fontFamily} placeholder="Padrão do sistema" hint="Ex.: Georgia, serif" onValueChange={(v) => onChange({ fontFamily: v })} />
    </Section>
  );
}
