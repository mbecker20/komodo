import { Component, JSX, Show } from "solid-js";
import { useTheme } from "../../state/ThemeProvider";
import { combineClasses } from "../../util/helpers";

const Input: Component<
  {
    onEdit?: (value: string) => void;
    onConfirm?: (value: string) => void;
    onEnter?: (value: string) => void;
    disabled?: boolean;
  } & JSX.InputHTMLAttributes<HTMLInputElement> &
    JSX.HTMLAttributes<HTMLDivElement>
> = (p) => {
  const { themeClass } = useTheme();
  return (
    <Show when={!p.disabled} fallback={<div {...p}>{p.value}</div>}>
      <input
        {...p}
        class={combineClasses(p.class, themeClass())}
        onInput={(e) => p.onEdit && p.onEdit(e.currentTarget.value)}
        onBlur={(e) => p.onConfirm && p.onConfirm(e.currentTarget.value)}
        onKeyDown={(e) => {
          if (e.key === "Enter") {
            p.onEnter
              ? p.onEnter(e.currentTarget.value)
              : e.currentTarget.blur();
          }
        }}
      />
    </Show>
  );
};

export default Input;
