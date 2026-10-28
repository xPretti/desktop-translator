import type { ResizeEdge } from "@/types/window";
import styles from "./ResizeHandles.module.css";

const EDGES: readonly ResizeEdge[] = ["n", "s", "e", "w", "ne", "nw", "se", "sw"];

interface IResizeHandlesProps {
  onResizeStart(edge: ResizeEdge): void;
}

/** Zonas invisíveis nas bordas e cantos que iniciam o redimensionamento nativo. */
export function ResizeHandles({ onResizeStart }: IResizeHandlesProps): React.JSX.Element {
  return (
    <>
      {EDGES.map((edge) => (
        <div
          key={edge}
          className={`${styles.handle} ${styles[edge] ?? ""}`}
          aria-hidden="true"
          onMouseDown={(event) => {
            if (event.button !== 0) return;
            event.preventDefault();
            event.stopPropagation();
            onResizeStart(edge);
          }}
        />
      ))}
    </>
  );
}
