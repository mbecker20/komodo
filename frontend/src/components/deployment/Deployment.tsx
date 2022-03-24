import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import Actions from "./Actions";
import s from "./deployment.module.css";
import DeploymentTabs from "./tabs/DeploymentTabs";

const Deployment: Component<{ id: string }> = (p) => {
	const { servers, deployments } = useAppState();
	const deployment = () => deployments.get(p.id);
	
	return (
    <Show when={deployment()}>
      <Grid class={s.Deployment}>
        {/* left / actions */}
        <Grid class={s.Left}>
          <Flex class={s.Header}>
            name:{" "}
            <div style={{ "font-weight": "bold" }}>{deployment()!.name}</div>
          </Flex>
          <Actions deployment={deployment()!} />
        </Grid>

        {/* right / tabs */}
        <DeploymentTabs />
      </Grid>
    </Show>
  );
}

export default Deployment;