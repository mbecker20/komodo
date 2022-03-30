import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { combineClasses } from "../../../util/helpers";
import Tabs from "../../util/tabs/Tabs";
import s from "../build.module.css";
import Config from "./config/Config";
import { ConfigProvider } from "./config/Provider";

const BuildTabs: Component<{}> = (p) => {
  const { builds, selected } = useAppState();
  const build = () => builds.get(selected.id())!;
  return (
    <Show when={build()}>
      <ConfigProvider build={build()}>
        <Tabs
          containerClass={combineClasses(s.Card, s.Tabs, "shadow")}
          tabs={[
            {
              title: "config",
              element: <Config />,
            },
          ]}
          localStorageKey="build-tab"
        />
      </ConfigProvider>
    </Show>
  );
};

export default BuildTabs;
