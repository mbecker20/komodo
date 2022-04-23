import { Component, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useTheme } from "../../state/ThemeProvider";
import { combineClasses } from "../../util/helpers";
import NotFound from "../NotFound";
import Grid from "../util/layout/Grid";
import Loading from "../util/loading/Loading";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import DeploymentTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Deployment: Component<{}> = (p) => {
  const { servers, deployments, selected } = useAppState();
  const deployment = () => deployments.get(selected.id()!);
  const server = () => deployment() && servers.get(deployment()?.serverID!);
  const { themeClass } = useTheme();
  const { isMobile } = useAppDimensions();
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
            <Show when={!isMobile()}>
              <Updates />
            </Show>
          </Grid>
          {/* right / tabs */}
          <DeploymentTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

export default Deployment;
