import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import Tabs from "../../util/tabs/Tabs";
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
          containerClass="card tabs shadow"
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
