import { useParams } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import Tabs, { Tab } from "../../shared/tabs/Tabs";
import Config from "./config/Config";
import { ConfigProvider } from "./config/Provider";
import Owners from "./Owners";
import Stats from "./stats/Stats";

const ServerTabs: Component<{}> = (p) => {
  const { servers } = useAppState();
  const params = useParams();
  const { user } = useUser();
  const server = () => servers.get(params.id)!;
  return (
    <Show when={server()}>
      <ConfigProvider>
        <Tabs
          containerClass="card shadow"
          tabs={
            [
              {
                title: "config",
                element: () => <Config />,
              },
              {
                title: "stats",
                element: () => <Stats />,
              },
              user().admin && {
                title: "collaborators",
                element: () => <Owners />,
              },
            ].filter((e) => e) as Tab[]
          }
          localStorageKey="server-tab"
        />
      </ConfigProvider>
    </Show>
  );
};

export default ServerTabs;
