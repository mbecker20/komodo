import { Component, For, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { combineClasses, inPx } from "../../util/helpers";
import AddServer from "./AddServer";
import Grid from "../util/layout/Grid";
import Tabs from "../util/tabs/Tabs";
import Builds from "./builds/Builds";
import Server from "./server/Server";
import s from "./sidebar.module.scss";
import { TOPBAR_HEIGHT } from "../..";
import { useTheme } from "../../state/ThemeProvider";

const Sidebar: Component<{}> = () => {
  const { sidebar, servers } = useAppState();
  const { height } = useAppDimensions();
  const { permissions, username } = useUser();
  const { themeClass } = useTheme();
  const filteredServerIds = () =>
    servers
      .ids()
      ?.filter(
        (id) =>
          permissions() > 1 || servers.get(id)!.owners.includes(username()!)
      );
  return (
    <Show when={servers.loaded() && sidebar.open()}>
      <Tabs
        containerClass={combineClasses(s.Sidebar, "shadow", themeClass())}
        localStorageKey="sidebar-tab"
        tabsGap="0rem"
        tabs={[
          {
            title: "servers",
            element: (
              <Grid
                class={combineClasses(s.DeploymentsTabContent, "scroller")}
                style={{
                  "max-height": inPx(height() - TOPBAR_HEIGHT - 80),
                }}
              >
                <For each={filteredServerIds()}>{(id) => <Server id={id} />}</For>
                <Show when={permissions() > 1}>
                  <AddServer />
                </Show>
              </Grid>
            ),
          },
          {
            title: "builds",
            element: (
              <Grid
                class="scroller"
                style={{
                  height: "fit-content",
                  "max-height": inPx(height() - TOPBAR_HEIGHT - 120),
                  padding: "0rem 0.5rem",
                }}
              >
                <Builds />
              </Grid>
            ),
          },
        ]}
      />
    </Show>
  );
};

export default Sidebar;
