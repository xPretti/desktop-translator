import { callCommand } from "@/services/tauri";
import type { IServiceStatus } from "@/types/settings";
import type { WindowAction, WindowInteraction, WindowLabel } from "@/types/window";

export interface ISystemService {
  getServiceStatus(): Promise<IServiceStatus>;
  copyText(text: string): Promise<void>;
  openTranslationPopup(): Promise<void>;
  hidePopup(): Promise<void>;
  beginWindowInteraction(label: WindowLabel, kind: WindowInteraction): Promise<void>;
  windowControl(label: WindowLabel, action: WindowAction): Promise<void>;
  resizePopup(width: number, height: number): Promise<void>;
  openMainWindow(page?: "translate" | "settings"): Promise<void>;
  quitApp(): Promise<void>;
}

export const systemService: ISystemService = {
  getServiceStatus: () => callCommand<IServiceStatus>("get_service_status"),
  copyText: (text) => callCommand<void>("copy_text", { text }),
  openTranslationPopup: () => callCommand<void>("open_translation_popup"),
  hidePopup: () => callCommand<void>("hide_popup"),
  beginWindowInteraction: (label, kind) => callCommand<void>("begin_window_interaction", { label, kind }),
  windowControl: (label, action) => callCommand<void>("window_control", { label, action }),
  resizePopup: (width, height) => callCommand<void>("resize_popup", { width, height }),
  openMainWindow: (page) => callCommand<void>("open_main_window", { page: page ?? null }),
  quitApp: () => callCommand<void>("quit_app"),
};
