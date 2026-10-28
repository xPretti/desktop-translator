import { callCommand } from "@/services/tauri";
import type { IAppSettings, ISaveSettingsResult } from "@/types/settings";

export interface ISettingsService {
  load(): Promise<IAppSettings>;
  save(settings: IAppSettings): Promise<ISaveSettingsResult>;
  reset(): Promise<ISaveSettingsResult>;
}

export class TauriSettingsService implements ISettingsService {
  load(): Promise<IAppSettings> {
    return callCommand<IAppSettings>("get_settings");
  }
  save(settings: IAppSettings): Promise<ISaveSettingsResult> {
    return callCommand<ISaveSettingsResult>("save_settings", { settings });
  }
  reset(): Promise<ISaveSettingsResult> {
    return callCommand<ISaveSettingsResult>("reset_settings");
  }
}

export const settingsService: ISettingsService = new TauriSettingsService();
