import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { MainApp } from "@/app/MainApp";
import { FloatingButtonApp } from "@/features/translation/components/FloatingButtonApp";
import { PopupApp } from "@/features/translation/components/PopupApp";

/** O mesmo bundle serve às três janelas; o rótulo da janela define qual interface renderizar. */
export function App(): React.JSX.Element {
  const label = getCurrentWebviewWindow().label;
  if (label === "popup") return <PopupApp />;
  if (label === "floating-button") return <FloatingButtonApp />;
  return <MainApp />;
}
