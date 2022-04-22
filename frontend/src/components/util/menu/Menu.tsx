import {
  Component,
  createEffect,
  createSignal,
  JSX,
  JSXElement,
  Show,
} from "solid-js";
import { useTheme } from "../../../state/ThemeProvider";
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
  backgroundColor?: string;
}> = (p) => {
  const [buffer, set] = createSignal(p.show);
  createEffect(() => {
    if (p.show) {
      set(true);
    } else {
      setTimeout(() => {
        set(false);
      }, 350);
    }
  });
  const { themeClass } = useTheme();
  return (
    <div class={s.MenuContainer} style={p.containerStyle}>
      {p.target}
      <Show when={buffer()}>
        <div
          class={s.MenuBackground}
          style={{ "background-color": p.backgroundColor }}
          onClick={p.close}
        />
        <div
          class={combineClasses(
            p.menuClass,
            s.Menu,
            themeClass(),
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
