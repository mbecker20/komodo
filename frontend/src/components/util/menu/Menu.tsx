import {
  Component,
  createEffect,
  createSignal,
  JSX,
  JSXElement,
  Show,
} from "solid-js";
import { combineClasses } from "../../../util/helpers";
import { getPositionClass, Position } from "./helpers";
import s from "./menu.module.scss";

const Menu: Component<{
  content: JSXElement;
  target: JSXElement;
  show: boolean;
  close: () => void;
  position?: Position;
  padding?: string | number;
  menuClass?: string;
  menuStyle?: JSX.CSSProperties;
  containerStyle?: JSX.CSSProperties;
}> = (p) => {
  const [buffer, set] = createSignal(p.show);
  createEffect(() => {
    if (p.show) {
      set(true);
    } else {
      setTimeout(() => {
        set(false);
      }, 250);
    }
  });
  return (
    <div class={s.MenuContainer} style={p.containerStyle}>
      {p.target}
      <Show when={buffer()}>
        <div class={s.MenuBackground} onClick={p.close} />
        <div
          class={combineClasses(
            p.menuClass,
            s.Menu,
            "shadow",
            getPositionClass(p.position),
            p.show ? s.Enter : s.Exit
          )}
          style={{ padding: p.padding, ...p.menuStyle }}
          onClick={(e) => e.stopPropagation()}
        >
          {p.content}
        </div>
      </Show>
    </div>
  );
};

export default Menu;
