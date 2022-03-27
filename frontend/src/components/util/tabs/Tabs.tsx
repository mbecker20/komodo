import { Component, createSignal, For, JSX, JSXElement } from "solid-js";
import { combineClasses } from "../../../util/helpers";
import { useLocalStorage } from "../../../util/hooks";
import s from "./Tabs.module.css";

export type Tab = {
  title: string;
  titleElement?: JSXElement;
  element: JSXElement;
};

const Tabs: Component<{
  tabs: Tab[];
  defaultSelected?: string;
  localStorageKey?: string;
  tabStyle?: JSX.CSSProperties;
  titleElement?: JSXElement;
  titleStyle?: JSX.CSSProperties;
  containerClass?: string;
  containerStyle?: JSX.CSSProperties;
}> = (p) => {
  const def = p.defaultSelected ? p.defaultSelected : p.tabs[0].title;
  const [selected, set] = p.localStorageKey
    ? useLocalStorage(def, p.localStorageKey)
    : createSignal(def);
  const current = () =>
    p.tabs.filter((tab) => tab.title === selected())[0];
  const getClassName = (title: string) =>
    selected() === title ? combineClasses(s.Tab, s.Active) : s.Tab;
  return (
    <div class={combineClasses(s.Tabs, p.containerClass)} style={p.containerStyle}>
      <div class={s.TabTitles}>
        <For each={p.tabs}>
          {(tab) => (
            <button
              class={getClassName(tab.title)}
              style={p.tabStyle}
              onClick={() => set(tab.title)}
            >
              {tab.titleElement || tab.title}
            </button>
          )}
        </For>
        {p.titleElement}
      </div>
      {current().element}
    </div>
  );
};

export default Tabs;
