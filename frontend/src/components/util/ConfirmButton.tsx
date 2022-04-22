import { Component, createSignal, JSX } from "solid-js";
import Button from "./Button";

const ConfirmButton: Component<{
  onConfirm?: () => void;
  onFirstClick?: () => void;
  color?: "red" | "green" | "blue" | "orange" | "grey";
  style?: JSX.CSSProperties;
  confirm?: JSX.Element;
}> = (p) => {
  const [confirm, set] = createSignal(false);

  return (
    <Button
      class={p.color || "green"}
      style={p.style}
      onBlur={() => set(false)}
      onClick={(e) => {
        e.stopPropagation();
        if (confirm()) {
          p.onConfirm && p.onConfirm();
        } else {
          p.onFirstClick && p.onFirstClick();
        }
        set((confirm) => !confirm);
      }}
    >
      {confirm() ? p.confirm || "confirm" : p.children}
    </Button>
  );
};

export default ConfirmButton;
