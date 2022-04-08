import { Component, For, JSX, Show } from "solid-js";
import { useToggle } from "../../../util/hooks";
import Icon from "../Icon";
import { Position } from "./helpers";
import Menu from "./Menu";
import s from "./menu.module.scss";

const Selector: Component<{
  selected: string;
  items: string[];
  onSelect?: (item: string, index: number) => void;
  position?: Position;
  targetClass?: string;
  disabled?: boolean;
  disabledClass?: string;
  disabledStyle?: JSX.CSSProperties;
}> = (p) => {
  const [show, toggle] = useToggle();
  return (
    <Show
      when={!p.disabled}
      fallback={
        <div class={p.disabledClass} style={p.disabledStyle}>
          {p.selected}
        </div>
      }
    >
      <Menu
        show={show()}
        close={toggle}
        target={
          <button class={p.targetClass} onClick={toggle}>
            {p.selected}
            <Icon type="chevron-down" />
          </button>
        }
        content={
          <For each={p.items}>
            {(item, index) => (
              <button
                onClick={() => {
                  p.onSelect && p.onSelect(item, index());
                  toggle();
                }}
                style={{ width: "100%", "justify-content": "flex-end" }}
                class={s.SelectorItem}
              >
                {item}
              </button>
            )}
          </For>
        }
        position={p.position}
        padding="0.25rem"
      />
    </Show>
  );
};

export default Selector;
