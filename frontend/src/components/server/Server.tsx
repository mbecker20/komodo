import { Component, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useTheme } from "../../state/ThemeProvider";
import { combineClasses } from "../../util/helpers";
import NotFound from "../NotFound";
import Grid from "../util/layout/Grid";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import ServerTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Server: Component<{}> = (p) => {
  const { servers, selected } = useAppState();
  const server = () => servers.get(selected.id())!;
  const { themeClass } = useTheme();
  const { isMobile } = useAppDimensions();
  return (
    <Show when={server()} fallback={<NotFound type="server" />}>
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
          <ServerTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

export default Server;
