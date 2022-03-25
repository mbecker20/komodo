import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import Icon from "../util/icons/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import Actions from "./Actions";
import s from "./deployment.module.css";
import Tabs from "./tabs/Tabs";
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
          <Flex class={combineClasses(s.Header, "shadow")} justifyContent="space-between" alignItems="center">
            <Grid gap="0.1rem">
              <div class={s.ItemHeader}>{deployment()!.name}</div>
              <div>{server()!.name}</div>
            </Grid>
            <button>
              <Icon type="trash" />
            </button>
          </Flex>
          <Actions deployment={deployment()!} />
          <Updates deploymentID={p.id} />
        </Grid>
        {/* right / tabs */}
        <Tabs deployment={deployment()!} />
      </Grid>
    </Show>
  );
}

export default Deployment;