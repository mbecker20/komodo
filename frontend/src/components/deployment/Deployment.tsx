import { Component, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useTheme } from "../../state/ThemeProvider";
import { useUser } from "../../state/UserProvider";
import { combineClasses } from "../../util/helpers";
import NotFound from "../NotFound";
import Grid from "../util/layout/Grid";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import { ConfigProvider } from "./tabs/config/Provider";
import DeploymentTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Deployment: Component<{}> = (p) => {
  const { servers, deployments, selected } = useAppState();
  const deployment = () => deployments.get(selected.id()!);
  const server = () => deployment() && servers.get(deployment()?.serverID!);
  const { themeClass } = useTheme();
  const { isSemiMobile } = useAppDimensions();
  const { permissions, username } = useUser();
  const userCanUpdate = () => {
    if (permissions() > 1) {
      return true;
    } else if (
      permissions() > 0 &&
      deployment()!.owners.includes(username()!)
    ) {
      return true;
    } else {
      return false;
    }
  };
  return (
    <Show
      when={deployment() && server()}
      fallback={<NotFound type="deployment" />}
    >
      <ActionStateProvider>
        <Grid class={combineClasses("content", themeClass())}>
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
              <h2 class={combineClasses("card tabs shadow", themeClass())}>
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

export default Deployment;
