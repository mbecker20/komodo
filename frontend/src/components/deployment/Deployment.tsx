import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Grid from "../util/layout/Grid";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import DeploymentTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Deployment: Component<{}> = (p) => {
  const { servers, deployments, selected } = useAppState();
  const deployment = () => deployments.get(selected.id());
  const server = () => deployment() && servers.get(deployment()?.serverID!);
  return (
    <Show
      when={deployment() && server()}
      fallback={
        <Grid class="content">
          <div class="left-content">deployment at id not found</div>
        </Grid>
      }
    >
      <ActionStateProvider>
        <Grid class="content">
          {/* left / actions */}
          <Grid class="left-content">
            <Header />
            <Actions />
            <Updates />
          </Grid>
          {/* right / tabs */}
          <DeploymentTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

export default Deployment;
