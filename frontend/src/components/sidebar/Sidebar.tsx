import { Component, For, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { combineClasses, inPx } from "../../util/helpers";
import AddServer from "../create/Server";
import { TOPBAR_HEIGHT } from "../topbar/Topbar";
import Grid from "../util/layout/Grid";
import Tabs from "../util/tabs/Tabs";
import Builds from "./builds/Builds";
import Server from "./server/Server";
import s from "./sidebar.module.css";

const SIDEBAR_WIDTH = 350;

const Sidebar: Component<{}> = () => {
  const { sidebar, servers } = useAppState();
  const { height } = useAppDimensions();
  return (
    <Show when={servers.loaded() && sidebar.open()}>
      <Tabs
        containerClass={combineClasses(s.Sidebar, "shadow")}
        containerStyle={{
          width: inPx(SIDEBAR_WIDTH),
          height: inPx(height() - TOPBAR_HEIGHT),
        }}
        tabsGap="0rem"
        tabs={[
          {
            title: "deployments",
            element: (
              <Grid  style={{ height: "fit-content", padding: "0rem 1rem" }}>
                <For each={servers.ids()}>{(id) => <Server id={id} />}</For>
                <AddServer />
              </Grid>
            ),
          },
          {
            title: "builds",
            element: (
              <Grid style={{ height: "fit-content", padding: "0rem 1rem" }}>
                <Builds />
              </Grid>
            ),
          },
        ]}
        localStorageKey="sidebar-tab"
      />
    </Show>
  );
};

export default Sidebar;
