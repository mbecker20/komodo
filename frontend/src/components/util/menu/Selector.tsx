import { Component, For } from "solid-js";
import { useToggle } from "../../../util/hooks";
import Icon from "../icons/Icon";
import { Position } from "./helpers";
import Menu from "./Menu";
import s from "./Menu.module.css";

const Selector: Component<{
  selected: string;
  items: string[];
  onSelect?: (item: string, index: number) => void;
  position?: Position;
}> = (p) => {
  const [show, toggle] = useToggle();
  return (
    <Menu
      show={show()}
      target={
        <button onClick={toggle}>
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
  );
};

export default Selector;
