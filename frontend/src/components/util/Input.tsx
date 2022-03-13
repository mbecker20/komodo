import { Component, JSX } from "solid-js";

const Input: Component<
  {
    onEdit?: (value: string) => void;
    onConfirm?: (value: string) => void;
  } & JSX.InputHTMLAttributes<HTMLInputElement>
> = (p) => {
  return (
    <input
      {...p}
      onInput={(e) => p.onEdit && p.onEdit(e.currentTarget.value)}
      onBlur={(e) => p.onConfirm && p.onConfirm(e.currentTarget.value)}
      onKeyDown={(e) => e.key === "Enter" && e.currentTarget.blur()}
    />
  );
};

export default Input;
