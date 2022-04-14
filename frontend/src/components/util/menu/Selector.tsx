import {
  Component,
  createEffect,
  createSignal,
  For,
  JSX,
  Show,
} from "solid-js";
import { combineClasses } from "../../../util/helpers";
import { useToggle } from "../../../util/hooks";
import Icon from "../Icon";
import Input from "../Input";
import { Position } from "./helpers";
import Menu from "./Menu";
import s from "./menu.module.scss";

const Selector: Component<{
  selected: string;
  items: string[];
  onSelect?: (item: string, index: number) => void;
  position?: Position;
  targetClass?: string;
  targetStyle?: JSX.CSSProperties;
  disabled?: boolean;
  disabledClass?: string;
  disabledStyle?: JSX.CSSProperties;
  useSearch?: boolean;
  itemClass?: string;
  itemStyle?: JSX.CSSProperties;
}> = (p) => {
  const [show, toggle] = useToggle();
  const [search, setSearch] = createSignal("");
  let ref: HTMLInputElement | undefined;
  createEffect(() => {
    if (show()) ref?.focus();
  });
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
        close={() => {
          toggle();
          setSearch("");
        }}
        target={
          <button class={p.targetClass} onClick={toggle} style={p.targetStyle}>
            {p.selected}
            <Icon type="chevron-down" />
          </button>
        }
        content={
          <>
            <Show when={p.useSearch}>
              <Input
                ref={ref}
                placeholder="search"
                value={search()}
                onEdit={setSearch}
                style={{ "text-align": "end" }}
                onKeyDown={(e: any) => {
                  if (e.key === "Escape") {
                    toggle();
                    setSearch("");
                  }
                }}
              />
            </Show>
            <For
              each={
                p.useSearch
                  ? p.items.filter((item) => item.includes(search()))
                  : p.items
              }
            >
              {(item, index) => (
                <button
                  onClick={() => {
                    p.onSelect && p.onSelect(item, index());
                    toggle();
                  }}
                  style={{
                    width: "100%",
                    "justify-content": "flex-end",
                    ...p.itemStyle,
                  }}
                  class={combineClasses(p.itemClass, s.SelectorItem)}
                >
                  {item}
                </button>
              )}
            </For>
          </>
        }
        position={p.position}
        padding="0.25rem"
      />
    </Show>
  );
};

export default Selector;
