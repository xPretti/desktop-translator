import { listen } from "@tauri-apps/api/event";
import { createContext, useCallback, useEffect, useMemo, useState } from "react";
import type { ReactNode } from "react";
import { DEFAULT_SETTINGS } from "@/constants/defaultSettings";
import { settingsService } from "@/features/settings/services/settingsService";
import type { ISettingsService } from "@/features/settings/services/settingsService";
import type { IAppSettings, ISaveSettingsResult } from "@/types/settings";

export interface ISettingsContextValue {
  settings: IAppSettings;
  loaded: boolean;
  save(next: IAppSettings): Promise<ISaveSettingsResult>;
  reset(): Promise<ISaveSettingsResult>;
}

export const SettingsContext = createContext<ISettingsContextValue | null>(null);

interface ISettingsProviderProps {
  children: ReactNode;
  service?: ISettingsService;
}

/** Carrega as configurações do backend e as mantém sincronizadas entre todas as janelas. */
export function SettingsProvider({
  children,
  service = settingsService,
}: ISettingsProviderProps): React.JSX.Element {
  const [settings, setSettings] = useState<IAppSettings>(DEFAULT_SETTINGS);
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    let active = true;
    let unlisten: (() => void) | undefined;
    service
      .load()
      .then((value) => active && setSettings(value))
      .catch(() => undefined)
      .finally(() => active && setLoaded(true));
    listen<IAppSettings>("settings-changed", (event) => setSettings(event.payload)).then((fn) => {
      if (active) unlisten = fn;
      else fn();
    });
    return () => {
      active = false;
      unlisten?.();
    };
  }, [service]);

  const save = useCallback(
    async (next: IAppSettings) => {
      const result = await service.save(next);
      setSettings(result.settings);
      return result;
    },
    [service],
  );

  const reset = useCallback(async () => {
    const result = await service.reset();
    setSettings(result.settings);
    return result;
  }, [service]);

  const value = useMemo(() => ({ settings, loaded, save, reset }), [settings, loaded, save, reset]);
  return <SettingsContext.Provider value={value}>{children}</SettingsContext.Provider>;
}
