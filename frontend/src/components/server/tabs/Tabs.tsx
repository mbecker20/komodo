import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { combineClasses } from "../../../util/helpers";
import Tabs from "../../util/tabs/Tabs";
import Config from "./config/Config";
import { ConfigProvider } from "./config/Provider";

const ServerTabs: Component<{}> = (p) => {
  const { servers, selected } = useAppState();
  const server = () => servers.get(selected.id())!;
  return (
    <Show when={server()}>
      <ConfigProvider server={server()}>
        <Tabs
          containerClass="card tabs shadow"
					tabs={[
						{
							title: "config",
							element: <Config />
						}
					]}
          localStorageKey="server-tab"
        />
      </ConfigProvider>
    </Show>
  );
};

export default ServerTabs;
