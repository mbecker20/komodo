import { useParams } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { MAX_PAGE_WIDTH } from "../..";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { PermissionLevel } from "../../types";
import { combineClasses, getId } from "../../util/helpers";
import NotFound from "../NotFound";
import Grid from "../shared/layout/Grid";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import { ConfigProvider } from "./tabs/config/Provider";
import DeploymentTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Deployment2: Component<{}> = (p) => {
  const { servers, deployments } = useAppState();
  const { isSemiMobile, isMobile } = useAppDimensions();
  const params = useParams();
  const deployment = () => deployments.get(params.id);
  const server = () =>
    deployment() && servers.get(deployment()!.deployment.server_id);
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
            <Show when={!isMobile()}>
              <Updates />
            </Show>
          </Grid>
          <DeploymentTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

const Deployment: Component<{}> = (p) => {
  const { servers, deployments } = useAppState();
  const params = useParams();
  const deployment = () => deployments.get(params.id);
  const server = () => deployment() && servers.get(deployment()!.deployment.server_id);
  const { isSemiMobile } = useAppDimensions();
  const { user } = useUser();
  const userCanUpdate = () => user().admin || deployment()?.deployment.permissions![getId(user())] === PermissionLevel.Update;
  return (
    <Show
      when={deployment() && server()}
      fallback={<NotFound type="deployment" />}
    >
      <ActionStateProvider>
        <Grid class={combineClasses("content")}>
          {/* left / actions */}
          <Grid class="left-content">
            <Header />
            <Actions />
            <Show when={!isSemiMobile() && userCanUpdate()}>
              <Updates />
            </Show>
          </Grid>
          {/* right / tabs */}
          <Show
            when={userCanUpdate()}
            fallback={
              <h2 class={combineClasses("card tabs shadow")}>
                you do not have permission to view this deployment
              </h2>
            }
          >
            <ConfigProvider>
              <DeploymentTabs />
            </ConfigProvider>
          </Show>
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

export default Deployment2;
