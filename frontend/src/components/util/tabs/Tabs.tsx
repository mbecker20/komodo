import {
  Accessor,
  Component,
  createEffect,
  createMemo,
  createSignal,
  For,
  JSX,
  JSXElement,
} from "solid-js";
import { combineClasses } from "../../../util/helpers";
import { LocalStorageSetter, useLocalStorage } from "../../../util/hooks";
import Flex from "../layout/Flex";
import s from "./tabs.module.scss";

export type Tab = {
  title: string;
  titleElement?: JSXElement;
  element: JSXElement;
};

const Tabs: Component<{
  tabs: (Tab | undefined | false)[];
  defaultSelected?: string;
  localStorageKey?: string;
  tabsGap?: string;
  tabStyle?: JSX.CSSProperties;
  containerClass?: string;
  containerStyle?: JSX.CSSProperties;
}> = (p) => {
  const tabs = createMemo(() => p.tabs.filter(val => val) as Tab[])
  const def = p.defaultSelected ? p.defaultSelected : tabs()[0].title;
  const [selected, set] = p.localStorageKey
    ? useLocalStorage(def, p.localStorageKey)
    : createSignal(def);
  createEffect(() => {
    if (tabs().filter((tab) => tab.title === selected())[0] === undefined) {
      set(tabs()[0].title);
    }
  })
  return <ControlledTabs selected={selected} set={set} {...p} />;
};

export const ControlledTabs: Component<{
  tabs: (Tab | undefined | false)[];
  selected: Accessor<string>;
  set: LocalStorageSetter<string>;
  tabsGap?: string;
  tabStyle?: JSX.CSSProperties;
  containerClass?: string;
  containerStyle?: JSX.CSSProperties;
}> = (p) => {
  const tabs = createMemo(() => p.tabs.filter((val) => val) as Tab[]);
  const current = () => tabs().findIndex((tab) => tab.title === p.selected())
  const getClassName = (title: string) =>
    p.selected() === title ? combineClasses(s.Tab, s.Active) : s.Tab;
  return (
    <div
      class={combineClasses(s.Tabs, p.containerClass)}
      style={p.containerStyle}
    >
      <Flex
        gap={p.tabsGap || "0rem"}
        alignItems="center"
        justifyContent="space-evenly"
      >
        <For each={tabs()}>
          {(tab) => (
            <button
              class={getClassName(tab.title)}
              style={p.tabStyle}
              onClick={() => p.set(tab.title)}
            >
              {tab.titleElement || tab.title}
            </button>
          )}
        </For>
      </Flex>
      {tabs()[current()].element}
    </div>
  );
};

export default Tabs;
