import type { ReactNode } from "react";

interface ISvgProps {
  children: ReactNode;
  size?: number;
  viewBox?: string;
  filled?: boolean;
}

function Svg({ children, size = 16, viewBox = "0 0 24 24", filled = false }: ISvgProps): React.JSX.Element {
  return (
    <svg
      viewBox={viewBox}
      width={size}
      height={size}
      fill={filled ? "currentColor" : "none"}
      stroke={filled ? "none" : "currentColor"}
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
    >
      {children}
    </svg>
  );
}

export function IconTranslate({ size }: { size?: number }): React.JSX.Element {
  return (
    <Svg size={size}>
      <path d="m5 8 6 6" />
      <path d="m4 14 6-6 2-3" />
      <path d="M2 5h12" />
      <path d="M7 2h1" />
      <path d="m22 22-5-10-5 10" />
      <path d="M14 18h6" />
    </Svg>
  );
}

export function IconGear({ size }: { size?: number }): React.JSX.Element {
  return (
    <Svg size={size}>
      <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
      <circle cx="12" cy="12" r="3" />
    </Svg>
  );
}

export function IconSwap(): React.JSX.Element {
  return (
    <Svg size={16}>
      <path d="m16 3 4 4-4 4" />
      <path d="M20 7H4" />
      <path d="m8 21-4-4 4-4" />
      <path d="M4 17h16" />
    </Svg>
  );
}

export function IconX(): React.JSX.Element {
  return (
    <Svg size={16}>
      <path d="M18 6 6 18" />
      <path d="m6 6 12 12" />
    </Svg>
  );
}

export function IconCopy(): React.JSX.Element {
  return (
    <Svg size={14}>
      <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
      <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
    </Svg>
  );
}

export function IconRefresh(): React.JSX.Element {
  return (
    <Svg size={14}>
      <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8" />
      <path d="M21 3v5h-5" />
      <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16" />
      <path d="M8 16H3v5" />
    </Svg>
  );
}

export function IconCheck(): React.JSX.Element {
  return (
    <Svg size={14}>
      <path d="M20 6 9 17l-5-5" />
    </Svg>
  );
}

/* Ícones de janela (10 px, traço fino, como no Windows 11). */
function WinSvg({ children }: { children: ReactNode }): React.JSX.Element {
  return (
    <svg viewBox="0 0 10 10" width="10" height="10" fill="none" stroke="currentColor" strokeWidth="1" aria-hidden="true">
      {children}
    </svg>
  );
}

export function IconMinimize(): React.JSX.Element {
  return (
    <WinSvg>
      <path d="M0 5.5h10" />
    </WinSvg>
  );
}

export function IconMaximize(): React.JSX.Element {
  return (
    <WinSvg>
      <rect x="0.5" y="0.5" width="9" height="9" />
    </WinSvg>
  );
}

export function IconRestore(): React.JSX.Element {
  return (
    <WinSvg>
      <rect x="0.5" y="2.5" width="7" height="7" />
      <path d="M2.5 2.5v-2h7v7h-2" />
    </WinSvg>
  );
}

export function IconClose(): React.JSX.Element {
  return (
    <WinSvg>
      <path d="M0 0l10 10M10 0L0 10" />
    </WinSvg>
  );
}
