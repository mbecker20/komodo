import { useParams } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import SimpleTabs from "../../shared/tabs/SimpleTabs";
import { Tab } from "../../shared/tabs/Tabs";
import BuilderConfig from "./builder/BuilderConfig";
import BuildConfig from "./config/BuildConfig";
import Permissions from "./Permissions";
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
                title: "config",
                element: () => <BuildConfig />,
              },
              {
                title: "builder",
                element: () => <BuilderConfig />
              },
              user().admin && {
                title: "permissions",
                element: () => <Permissions />,
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
