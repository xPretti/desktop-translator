export type ThemeMode = "light" | "dark" | "auto";
export type PlacementSide = "below" | "above";
export type PlacementFallback = "flip" | "clamp";
export type AfterTranslation = "none" | "copy";
export type NewSelectionBehavior = "replace" | "close";
export type CloseAction = "tray" | "quit";

export interface IAppearanceSettings {
  theme: ThemeMode;
  maxWidth: number;
  maxHeight: number;
  minWidth: number;
  /** Cores vazias usam o padrão do tema atual. */
  borderColor: string;
  borderWidth: number;
  backgroundColor: string;
  textColor: string;
  fontSize: number;
  fontFamily: string;
  padding: number;
  borderRadius: number;
  opacity: number;
}

export interface IPositioningSettings {
  popupSide: PlacementSide;
  buttonSide: PlacementSide;
  offsetX: number;
  offsetY: number;
  minDistance: number;
  buttonDistance: number;
  screenMargin: number;
  fallback: PlacementFallback;
}

export interface ITranslationSettings {
  sourceLanguage: string;
  /** Idiomas de destino em ordem de prioridade (1 a 5). Se o texto já estiver no primeiro, usa o próximo. */
  targetLanguages: string[];
  autoTranslate: boolean;
  maxTextLength: number;
  timeoutSeconds: number;
  cacheEnabled: boolean;
  clipboardFallback: boolean;
  afterTranslation: AfterTranslation;
  translationEnabled: boolean;
  captureEnabled: boolean;
}

export interface IShortcutSettings {
  toggleCapture: string;
  openMain: string;
  translateSelection: string;
}

export interface IBehaviorSettings {
  launchAtStartup: boolean;
  startMinimized: boolean;
  minimizeToTray: boolean;
  closeAction: CloseAction;
  popupAutoClose: boolean;
  popupAutoCloseSeconds: number;
  buttonAutoHideSeconds: number;
  closeOnOutsideClick: boolean;
  newSelectionBehavior: NewSelectionBehavior;
  shortcuts: IShortcutSettings;
}

export interface IAppSettings {
  appearance: IAppearanceSettings;
  positioning: IPositioningSettings;
  translation: ITranslationSettings;
  behavior: IBehaviorSettings;
}

export interface ISaveSettingsResult {
  settings: IAppSettings;
  warnings: string[];
}

export interface IServiceStatus {
  translationEnabled: boolean;
  captureEnabled: boolean;
  captureSupported: boolean;
}
