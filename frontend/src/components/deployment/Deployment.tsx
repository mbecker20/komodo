import { useParams } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { PermissionLevel } from "../../types";
import Description from "../Description";
import NotFound from "../NotFound";
import Grid from "../shared/layout/Grid";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import DeploymentTabs from "./tabs/Tabs";
import Updates from "./Updates";

const POLLING_RATE = 10000;
// let interval = -1;

const Deployment: Component<{}> = (p) => {
  const { user, user_id } = useUser();
  const { servers, deployments } = useAppState();
  const { isSemiMobile } = useAppDimensions();
  const params = useParams();
  const deployment = () => deployments.get(params.id);
  const server = () =>
    deployment() && servers.get(deployment()!.deployment.server_id);
  const userCanUpdate = () =>
    user().admin ||
    deployment()?.deployment.permissions![user_id()] === PermissionLevel.Update;
  // clearInterval(interval);
  // interval = setInterval(async () => {
  //   if (server()?.status === ServerStatus.Ok) {
  //     const deployment = await client.get_deployment(params.id);
  //     deployments.update(deployment);
  //   }
  // }, POLLING_RATE);
  // onCleanup(() => clearInterval(interval));
  return (
    <Show
      when={deployment() && server()}
      fallback={<NotFound type="deployment" loaded={deployments.loaded()} />}
    >
      <ActionStateProvider>
        <Grid
          style={{
            width: "100%",
            "box-sizing": "border-box",
          }}
        >
          <Header />
          <Grid
            style={{ width: "100%" }}
            gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}
          >
            <Grid style={{ "flex-grow": 1, "grid-auto-rows": "auto 1fr" }}>
              <Description
                target={{ type: "Deployment", id: params.id }}
                name={deployment()?.deployment.name!}
                description={deployment()?.deployment.description}
                userCanUpdate={userCanUpdate()}
              />
              <Actions />
            </Grid>
            <Show when={!isSemiMobile()}>
              <Updates />
            </Show>
          </Grid>
          <DeploymentTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

export default Deployment;
