import { Component, createSignal, JSX } from "solid-js";

const ConfirmButton: Component<{
  onConfirm: () => void;
  color?: "red" | "green" | "blue" | "orange";
  style?: JSX.CSSProperties;
}> = (p) => {
  const [confirm, set] = createSignal(false);

  return (
    <button
      class={p.color || "green"}
      style={p.style}
      onBlur={() => set(false)}
      onClick={(e) => {
        e.stopPropagation();
        if (confirm()) {
          p.onConfirm();
        }
        set(confirm => !confirm);
      }}
    >
      {confirm() ? "confirm" : p.children}
    </button>
  );
};

export default ConfirmButton;
