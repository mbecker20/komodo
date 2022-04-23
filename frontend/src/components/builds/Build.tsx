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
import BuildTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Build: Component<{}> = (p) => {
  const { builds, selected } = useAppState();
  const build = () => builds.get(selected.id())!;
  const { themeClass } = useTheme();
  const { isMobile } = useAppDimensions();
  return (
    <Show when={build()} fallback={<NotFound type="build" />}>
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
          <BuildTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

export default Build;
