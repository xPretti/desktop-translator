import { Section } from "@/components/layout/Section";
import { NumberField, TextField } from "@/components/ui/Field";
import { Select } from "@/components/ui/Select";
import { Switch } from "@/components/ui/Switch";
import type { ISectionProps } from "@/features/settings/types/sectionProps";
import type { CloseAction, IBehaviorSettings, IShortcutSettings, NewSelectionBehavior } from "@/types/settings";

const CLOSE_ACTIONS = [
  { value: "tray", label: "Manter na bandeja do sistema" },
  { value: "quit", label: "Encerrar o aplicativo" },
];
const NEW_SELECTION = [
  { value: "replace", label: "Manter até traduzir a nova seleção" },
  { value: "close", label: "Fechar o pop-up anterior" },
];

export function BehaviorSection({ value, errors, prefix, onChange }: ISectionProps<IBehaviorSettings>): React.JSX.Element {
  const shortcut = (key: keyof IShortcutSettings, label: string): React.JSX.Element => (
    <TextField
      label={label}
      value={value.shortcuts[key]}
      error={errors[`${prefix}.shortcuts.${key}`]}
      hint="Ex.: Ctrl+Alt+T. Vazio desativa."
      onValueChange={(v) => onChange({ shortcuts: { ...value.shortcuts, [key]: v } })}
    />
  );
  return (
    <Section title="Comportamento e atalhos">
      <Switch label="Iniciar com o Windows" checked={value.launchAtStartup} onCheckedChange={(v) => onChange({ launchAtStartup: v })} />
      <Switch label="Iniciar minimizado" checked={value.startMinimized} onCheckedChange={(v) => onChange({ startMinimized: v })} />
      <Switch label="Minimizar para a bandeja" checked={value.minimizeToTray} onCheckedChange={(v) => onChange({ minimizeToTray: v })} />
      <Select label="Ao fechar a janela principal" options={CLOSE_ACTIONS} value={value.closeAction} onValueChange={(v) => onChange({ closeAction: v as CloseAction })} />
      <Select label="Nova seleção com pop-up aberto" options={NEW_SELECTION} value={value.newSelectionBehavior} onValueChange={(v) => onChange({ newSelectionBehavior: v as NewSelectionBehavior })} />
      <Switch label="Fechar ao clicar fora" checked={value.closeOnOutsideClick} onCheckedChange={(v) => onChange({ closeOnOutsideClick: v })} />
      <Switch label="Fechar o pop-up automaticamente" checked={value.popupAutoClose} onCheckedChange={(v) => onChange({ popupAutoClose: v })} />
      <NumberField label="Tempo para fechar o pop-up (s)" value={value.popupAutoCloseSeconds} error={errors[`${prefix}.popupAutoCloseSeconds`]} onValueChange={(v) => onChange({ popupAutoCloseSeconds: v })} />
      <NumberField label="Ocultar botão após (s, 0 = nunca)" value={value.buttonAutoHideSeconds} error={errors[`${prefix}.buttonAutoHideSeconds`]} onValueChange={(v) => onChange({ buttonAutoHideSeconds: v })} />
      {shortcut("translateSelection", "Atalho: traduzir seleção")}
      {shortcut("toggleCapture", "Atalho: ativar/desativar captura")}
      {shortcut("openMain", "Atalho: abrir janela principal")}
    </Section>
  );
}
