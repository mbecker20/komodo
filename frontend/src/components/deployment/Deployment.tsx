import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Grid from "../util/layout/Grid";
import s from "./deployment.module.css";
import DeploymentTabs from "./tabs/DeploymentTabs";

const Deployment: Component<{ id: string }> = (p) => {
	const { servers, deployments } = useAppState();
	const deployment = () => deployments.get(p.id);
	
	return (
    <Show when={deployment()}>
      <Grid class={s.Deployment}>
        {/* left / actions */}
        <Grid>
          <div>name: {deployment()!.name}</div>
        </Grid>

        {/* right / tabs */}
        <DeploymentTabs />
      </Grid>
    </Show>
  );
}

export default Deployment;