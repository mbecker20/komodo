import { Component, createMemo, For, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { getId } from "../../util/helpers";
import Grid from "../shared/layout/Grid";
import { useUser } from "../../state/UserProvider";
import { PermissionLevel } from "../../types";
import { NewDeployment } from "../New";
import Deployment from "./Deployment";
import { useAppState } from "../../state/StateProvider";

const ServerChildren: Component<{ id: string }> = (p) => {
  const { user } = useUser();
  const { isSemiMobile } = useAppDimensions();
  const { servers, deployments } = useAppState();
  const server = () => servers.get(p.id);
  const deploymentIDs = createMemo(() => {
    return (deployments.loaded() &&
      deployments
        .ids()!
        .filter(
          (id) => deployments.get(id)?.deployment.server_id === p.id
        )) as string[];
  });
  return (
    <div class="card shadow">
      <Grid
        gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}
        gap="0.5rem"
      >
        <For each={deploymentIDs()}>{(id) => <Deployment id={id} />}</For>
        <Show
          when={
            user().admin ||
            server()?.server.permissions![getId(user())] ===
              PermissionLevel.Update
          }
        >
          <NewDeployment serverID={p.id} />
        </Show>
      </Grid>
    </div>
  );
};

export default ServerChildren;
