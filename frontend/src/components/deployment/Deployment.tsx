import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Grid from "../util/layout/Grid";
import Actions from "./Actions";
import s from "./deployment.module.css";
import Header from "./Header";
import Tabs from "./tabs/Tabs";
import Updates from "./Updates";

const Deployment: Component<{}> = (p) => {
  const { servers, deployments, selected } = useAppState();
  const deployment = () => deployments.get(selected.id());
  const server = () => deployment() && servers.get(deployment()?.serverID!);
  return (
    <Show when={deployment() && server()}>
      <Grid class={s.Deployment}>
        {/* left / actions */}
        <Grid class={s.Left}>
          <Header />
          <Actions />
          <Updates />
        </Grid>
        {/* right / tabs */}
        <Tabs deployment={deployment()!} />
      </Grid>
    </Show>
  );
};

export default Deployment;
