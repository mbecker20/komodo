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
  padding?: string;
  contentStyle?: JSX.CSSProperties;

}> = (p) => {
  const [show, set] = createSignal(false);
  const [buffer, setBuffer] = createSignal(false);
  let timeout: NodeJS.Timeout;
  createEffect(() => {
    clearTimeout(timeout);
    if (show()) {
      setBuffer(true);
    } else {
      timeout = setTimeout(() => {
        setBuffer(false);
      }, 500);
    }
  });
  return (
    <div
      class={s.HoverMenuTarget}
      onMouseEnter={() => set(true)}
      onMouseLeave={() => set(false)}
      onTouchStart={() => set((show) => !show)}
      // onClick={(e) => e.stopPropagation()}
    >
      {p.target}
      <Show when={buffer()}>
        <div
          class={combineClasses(
            getPositionClass(p.position),
            s.HoverMenu,
            show() ? s.Enter : s.Exit
          )}
          onMouseOut={() => {
            set(false);
          }}
          onMouseEnter={(e) => {
            set(false)
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
