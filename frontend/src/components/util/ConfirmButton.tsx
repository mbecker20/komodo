import { Component, createSignal, JSX } from "solid-js";

const ConfirmButton: Component<{
  onConfirm: () => void;
  color: "red" | "green" | "blue";
  style?: JSX.CSSProperties;
}> = (p) => {
  const [confirm, set] = createSignal(false);

  return (
    <button
      className={p.color}
      style={p.style}
      onBlur={() => set(false)}
      onClick={(e) => {
        e.stopPropagation();
        confirm() ? p.onConfirm() : set(true);
      }}
    >
      {confirm() ? "Confirm" : p.children}
    </button>
  );
};

export default ConfirmButton;
