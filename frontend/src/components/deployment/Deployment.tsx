import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import Actions from "./Actions";
import s from "./deployment.module.css";
import DeploymentTabs from "./tabs/DeploymentTabs";
import Updates from "./updates/Updates";

const Deployment: Component<{ id: string }> = (p) => {
	const { servers, deployments } = useAppState();
	const deployment = () => deployments.get(p.id);
  const server = () => deployment() && servers.get(deployment()?.serverID!);
	
	return (
    <Show when={deployment() && server()}>
      <Grid class={s.Deployment}>
        {/* left / actions */}
        <Grid class={s.Left}>
          <Flex class={s.Header}>
            <Grid gap="0.1rem">
              <div class={s.ItemHeader}>{deployment()!.name}</div>
              <div>{server()!.name}</div>
            </Grid>
          </Flex>
          <Actions deployment={deployment()!} />
          <Updates deploymentID={p.id} />
        </Grid>
        {/* right / tabs */}
        <DeploymentTabs deployment={deployment()!} />
      </Grid>
    </Show>
  );
}

export default Deployment;