import {
  Component,
  createEffect,
  createSignal,
  JSX,
  JSXElement,
  Show,
} from "solid-js";
import { combineClasses } from "../../../util/helpers";
import { getPositionClass } from "./helpers";
import { Position } from "./helpers";
import s from "./menu.module.scss";

const HoverMenu: Component<{
  target: JSXElement;
  content: JSXElement;
  position?: Position;
  interactive?: boolean;
  padding?: string;
  contentStyle?: JSX.CSSProperties;
}> = (p) => {
  const [show, set] = createSignal(false);
  const [buffer, setBuffer] = createSignal(false);
  createEffect(() => {
    if (show()) {
      setBuffer(true);
    } else {
      setTimeout(() => {
        setBuffer(false);
      }, 500);
    }
  });
  return (
    <div
      class={s.HoverMenuTarget}
      onMouseEnter={() => set(true)}
      onMouseLeave={() => {
        if (!p.interactive) set(false);
      }}
      onTouchStart={() => set((show) => !show)}
      onClick={(e) => e.stopPropagation()}
    >
      {p.target}
      <Show when={buffer()}>
        <div
          class={combineClasses(
            s.HoverMenu,
            getPositionClass(p.position),
            show() ? s.Enter : s.Exit
          )}
          onMouseOut={() => {
            if (p.interactive) set(false);
          }}
          onMouseEnter={(e) => {
            e.stopPropagation();
          }}
          style={{ ...p.contentStyle, padding: p.padding }}
        >
          {p.content}
        </div>
      </Show>
    </div>
  );
};

export default HoverMenu;
