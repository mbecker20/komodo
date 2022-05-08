import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import Tabs from "../../util/tabs/Tabs";
import Config from "./config/Config";
import { ConfigProvider } from "./config/Provider";
import Owners from "./Owners";
import Stats from "./Stats";

const ServerTabs: Component<{}> = (p) => {
  const { servers, selected } = useAppState();
  const { username, permissions } = useUser();
  const server = () => servers.get(selected.id())!;
  const userCanUpdate = () => {
    if (permissions() > 1) {
      return true;
    } else if (permissions() > 0 && server().owners.includes(username()!)) {
      return true;
    } else {
      return false;
    }
  };
  return (
    <Show when={server()}>
      <ConfigProvider>
        <Tabs
          containerClass="card shadow"
          tabs={[
            {
              title: "config",
              element: <Config />,
            },
            {
              title: "stats",
              element: <Stats />
            },
            userCanUpdate() && {
              title: "collaborators",
              element: <Owners />,
            },
          ]}
          localStorageKey="server-tab"
        />
      </ConfigProvider>
    </Show>
  );
};

export default ServerTabs;
