import { useEffect, useState } from "react";
import { systemService } from "@/features/translation/services/systemService";
import { useSettings } from "@/hooks/useSettings";
import type { IServiceStatus } from "@/types/settings";

/** Consulta o backend sempre que as configurações mudam. */
export function useServiceStatus(): IServiceStatus | null {
  const { settings } = useSettings();
  const [status, setStatus] = useState<IServiceStatus | null>(null);
  useEffect(() => {
    let active = true;
    systemService
      .getServiceStatus()
      .then((value) => active && setStatus(value))
      .catch(() => undefined);
    return () => {
      active = false;
    };
  }, [settings]);
  return status;
}
