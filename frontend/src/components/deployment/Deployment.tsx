import { useParams } from "@solidjs/router";
import { Component, onCleanup, Show } from "solid-js";
import { client } from "../..";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { ServerStatus } from "../../types";
import NotFound from "../NotFound";
import Grid from "../shared/layout/Grid";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import DeploymentTabs from "./tabs/Tabs";
import Updates from "./Updates";

const POLLING_RATE = 10000;
let interval = -1;

const Deployment: Component<{}> = (p) => {
  const { servers, deployments } = useAppState();
  const { isSemiMobile } = useAppDimensions();
  const params = useParams();
  const deployment = () => deployments.get(params.id);
  const server = () =>
    deployment() && servers.get(deployment()!.deployment.server_id);
  clearInterval(interval);
  interval = setInterval(async () => {
    if (server()?.status === ServerStatus.Ok) {
      const deployment = await client.get_deployment(params.id);
      deployments.update(deployment);
    }
  }, POLLING_RATE);
  onCleanup(() => clearInterval(interval));
  return (
    <Show
      when={deployment() && server()}
      fallback={<NotFound type="deployment" />}
    >
      <ActionStateProvider>
        <Grid
          style={{
            width: "100%",
            "box-sizing": "border-box",
          }}
        >
          <Grid
            style={{ width: "100%" }}
            gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}
          >
            <Grid style={{ "flex-grow": 1, "grid-auto-rows": "auto 1fr" }}>
              <Header />
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
