import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Grid from "../util/layout/Grid";
import Actions from "./Actions";
import s from "./deployment.module.css";
import Header from "./Header";
import Tabs from "./tabs/Tabs";
import Updates from "./Updates";

const Deployment: Component<{ id: string }> = (p) => {
  const { servers, deployments } = useAppState();
  const deployment = () => deployments.get(p.id);
  const server = () => deployment() && servers.get(deployment()?.serverID!);
  return (
    <Show when={deployment() && server()}>
      <Grid class={s.Deployment}>
        {/* left / actions */}
        <Grid class={s.Left}>
          <Header id={p.id} />
          <Actions deployment={deployment()!} />
          <Updates deploymentID={p.id} />
        </Grid>
        {/* right / tabs */}
        <Tabs deployment={deployment()!} />
      </Grid>
    </Show>
  );
};

export default Deployment;
