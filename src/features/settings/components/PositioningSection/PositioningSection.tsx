import { Section } from "@/components/layout/Section";
import { NumberField } from "@/components/ui/Field";
import { Select } from "@/components/ui/Select";
import type { ISectionProps } from "@/features/settings/types/sectionProps";
import type { IPositioningSettings, PlacementFallback, PlacementSide } from "@/types/settings";

const SIDES = [
  { value: "below", label: "Abaixo da seleção" },
  { value: "above", label: "Acima da seleção" },
];
const FALLBACKS = [
  { value: "flip", label: "Inverter para o outro lado" },
  { value: "clamp", label: "Manter e ajustar à tela" },
];

export function PositioningSection({ value, errors, prefix, onChange }: ISectionProps<IPositioningSettings>): React.JSX.Element {
  const num = (key: keyof IPositioningSettings, label: string): React.JSX.Element => (
    <NumberField label={label} value={value[key] as number} error={errors[`${prefix}.${key}`]} onValueChange={(v) => onChange({ [key]: v })} />
  );
  return (
    <Section title="Posicionamento" description="Valores em pixels físicos da tela; considera múltiplos monitores e DPI.">
      <Select label="Posição do pop-up" options={SIDES} value={value.popupSide} onValueChange={(v) => onChange({ popupSide: v as PlacementSide })} />
      <Select label="Posição do botão" options={SIDES} value={value.buttonSide} onValueChange={(v) => onChange({ buttonSide: v as PlacementSide })} />
      <Select label="Sem espaço disponível" options={FALLBACKS} value={value.fallback} onValueChange={(v) => onChange({ fallback: v as PlacementFallback })} />
      {num("offsetX", "Deslocamento X")}
      {num("offsetY", "Deslocamento Y")}
      {num("minDistance", "Distância mínima do pop-up")}
      {num("buttonDistance", "Distância do botão")}
      {num("screenMargin", "Margem das bordas da tela")}
    </Section>
  );
}
