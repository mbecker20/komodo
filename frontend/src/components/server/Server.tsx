import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Grid from "../util/layout/Grid";
import Actions from "./Actions";
import Header from "./Header";
import ServerTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Server: Component<{}> = (p) => {
	const { servers, selected } = useAppState();
  const server = () => servers.get(selected.id())!;
	return (
    <Show when={server()}>
      <Grid class="content">
        {/* left / actions */}
        <Grid class="left-content">
          <Header />
          <Actions />
          <Updates />
        </Grid>
        {/* right / tabs */}
        <ServerTabs />
      </Grid>
    </Show>
  );
}

export default Server;