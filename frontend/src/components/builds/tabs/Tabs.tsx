import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { combineClasses } from "../../../util/helpers";
import Tabs from "../../util/tabs/Tabs";
import s from "../build.module.css";
import BuildConfig from "./build-config/BuildConfig";
import GitConfig from "./git-config/GitConfig";
import { ConfigProvider } from "./Provider";

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
              title: "repo",
              element: <GitConfig />,
            },
            {
              title: "build",
              element: <BuildConfig />
            }
          ]}
          localStorageKey="build-tab"
        />
      </ConfigProvider>
    </Show>
  );
};

export default BuildTabs;
