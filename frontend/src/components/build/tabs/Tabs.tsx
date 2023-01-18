import { useParams } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import { PermissionLevel } from "../../../types";
import { getId } from "../../../util/helpers";
import SimpleTabs from "../../shared/tabs/SimpleTabs";
import { Tab } from "../../shared/tabs/Tabs";
import BuildConfig from "./build-config/BuildConfig";
import GitConfig from "./git-config/GitConfig";
import Owners from "./Permissions";
import { ConfigProvider } from "./Provider";

const BuildTabs: Component<{}> = (p) => {
  const { builds } = useAppState();
  const params = useParams();
  const { user } = useUser();
  const build = () => builds.get(params.id)!;
  return (
    <Show when={build()}>
      <ConfigProvider>
        <SimpleTabs
          containerClass="card shadow"
          tabs={
            [
              {
                title: "repo",
                element: () => <GitConfig />,
              },
              {
                title: "build",
                element: () => <BuildConfig />,
              },
              user().admin && {
                title: "collaborators",
                element: () => <Owners />,
              },
            ].filter((e) => e) as Tab[]
          }
          localStorageKey="build-tab"
        />
      </ConfigProvider>
    </Show>
  );
};

export default BuildTabs;
