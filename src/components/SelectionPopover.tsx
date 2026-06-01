import { useEffect, useRef } from "react";

interface Props {
  x: number;
  y: number;
  onCopy: () => void;
  onSave: () => void;
  onDismiss: () => void;
}

/**
 * Tiny floating menu that appears above a text selection.
 * Position is in viewport coordinates (clientX/clientY).
 */
export default function SelectionPopover({
  x,
  y,
  onCopy,
  onSave,
  onDismiss,
}: Props) {
  const ref = useRef<HTMLDivElement | null>(null);

  // Prevent the mousedown on the popover from collapsing the selection.
  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    const handler = (e: MouseEvent) => e.preventDefault();
    el.addEventListener("mousedown", handler);
    return () => el.removeEventListener("mousedown", handler);
  }, []);

  return (
    <div
    ref={ref}
      className="selection-popover"
      style={{ left: x, top: y }}
      role="menu"
      onClick={(e) => e.stopPropagation()}
    >
      <button onClick={onCopy} title="Copy">
        Copy
      </button>
      <span className="sep" />
      <button
        onClick={() => {
          onSave();
          onDismiss();
        }}
        title="Save to notebook"
      >
        Save
      </button>
    </div>
  );
}