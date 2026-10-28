import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { ResizeHandles } from "@/components/layout/ResizeHandles";
import { TitleBar } from "@/components/layout/TitleBar";
import { IconButton } from "@/components/ui/IconButton";
import { IconGear, IconTranslate } from "@/components/ui/Icons";
import { SettingsModal } from "@/features/settings/components/SettingsModal";
import { TranslationPanel } from "@/features/translation/components/TranslationPanel";
import { useSettings } from "@/hooks/useSettings";
import { useTheme } from "@/hooks/useTheme";
import { useWindowControls } from "@/hooks/useWindowControls";
import styles from "./MainApp.module.css";

type Page = "translate" | "settings";

export function MainApp(): React.JSX.Element {
  const { settings } = useSettings();
  const controls = useWindowControls("main");
  const [settingsOpen, setSettingsOpen] = useState(false);
  useTheme(settings.appearance.theme);

  // Atalhos da bandeja: "settings" abre o modal; "translate" volta à página inicial.
  useEffect(() => {
    let active = true;
    let unlisten: (() => void) | undefined;
    listen<Page>("navigate", (event) => setSettingsOpen(event.payload === "settings")).then((fn) => {
      if (active) unlisten = fn;
      else fn();
    });
    return () => {
      active = false;
      unlisten?.();
    };
  }, []);

  return (
    <div className={styles.window}>
      <TitleBar
        icon={<IconTranslate size={16} />}
        title="Desktop Translator"
        maximized={controls.maximized}
        onMinimize={controls.minimize}
        onToggleMaximize={controls.toggleMaximize}
        onClose={controls.close}
        onDragStart={controls.beginDrag}
        actions={
          <IconButton title="Configurações" onClick={() => setSettingsOpen(true)}>
            <IconGear size={18} />
          </IconButton>
        }
      />
      <div className={styles.body}>
        <main className={styles.content}>
          <TranslationPanel />
        </main>
        {settingsOpen ? <SettingsModal onClose={() => setSettingsOpen(false)} /> : null}
      </div>
      {controls.maximized ? null : <ResizeHandles onResizeStart={controls.beginResize} />}
    </div>
  );
}
