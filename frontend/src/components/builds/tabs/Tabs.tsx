import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import Tabs from "../../util/tabs/Tabs";
import BuildConfig from "./build-config/BuildConfig";
import GitConfig from "./git-config/GitConfig";
import Owners from "./Owners";
import { ConfigProvider } from "./Provider";

const BuildTabs: Component<{}> = (p) => {
  const { builds, selected } = useAppState();
  const { username, permissions } = useUser();
  const build = () => builds.get(selected.id())!;
  const userCanUpdate = () => {
    if (permissions() > 1) {
      return true;
    } else if (permissions() > 0 && build().owners.includes(username()!)) {
      return true;
    } else {
      return false;
    }
  };
  return (
    <Show when={build()}>
      <ConfigProvider>
        <Tabs
          containerClass="card tabs shadow"
          tabs={[
            {
              title: "repo",
              element: <GitConfig />,
            },
            {
              title: "build",
              element: <BuildConfig />,
            },
            userCanUpdate() && {
              title: "collaborators",
              element: <Owners />,
            },
          ]}
          localStorageKey="build-tab"
        />
      </ConfigProvider>
    </Show>
  );
};

export default BuildTabs;
