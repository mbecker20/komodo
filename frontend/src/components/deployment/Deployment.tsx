import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Grid from "../util/layout/Grid";
import Actions from "./Actions";
import s from "./deployment.module.css";
import Header from "./Header";
import DeploymentTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Deployment: Component<{}> = (p) => {
  const { servers, deployments, selected } = useAppState();
  const deployment = () => deployments.get(selected.id());
  const server = () => deployment() && servers.get(deployment()?.serverID!);
  return (
    <Show when={deployment() && server()}>
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
    </Show>
  );
};

export default Deployment;
