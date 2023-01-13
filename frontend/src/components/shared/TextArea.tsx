import { Component, JSX, Show } from "solid-js";

const TextArea: Component<
  {
    onEdit?: (value: string) => void;
    onConfirm?: (value: string) => void;
    onEnter?: (value: string) => void;
    disabled?: boolean;
  } & JSX.InputHTMLAttributes<HTMLTextAreaElement> &
    JSX.HTMLAttributes<HTMLDivElement>
> = (p) => {
  return (
    <Show when={!p.disabled} fallback={<div {...p}>{p.value}</div>}>
      <textarea
        {...p}
        onInput={(e) => p.onEdit && p.onEdit(e.currentTarget.value)}
        onBlur={(e) => p.onConfirm && p.onConfirm(e.currentTarget.value)}
        onKeyDown={(e) => {
          if (e.key === "Enter" && p.onEnter) {
            p.onEnter(e.currentTarget.value);
          }
        }}
      />
    </Show>
  );
};

export default TextArea;
