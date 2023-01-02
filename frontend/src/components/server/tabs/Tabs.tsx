import { useParams } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import { PermissionLevel } from "../../../types";
import { getId } from "../../../util/helpers";
import Tabs, { Tab } from "../../shared/tabs/Tabs";
import Config from "./config/Config";
import { ConfigProvider } from "./config/Provider";
import Owners from "./Owners";
import Stats from "./stats/Stats";

const ServerTabs: Component<{}> = (p) => {
  const { servers } = useAppState();
  const { id } = useParams();
  const { user } = useUser();
  const server = () => servers.get(id)!;
  const userCanUpdate = () =>
    user().admin ||
    server()!.server.permissions![getId(user())] === PermissionLevel.Update;
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
              userCanUpdate() && {
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
