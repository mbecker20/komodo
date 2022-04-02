import { Component, For, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
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
  const { sidebar, servers, selected } = useAppState();
  const { height } = useAppDimensions();
  const { permissions } = useUser();
  return (
    <Show when={servers.loaded() && sidebar.open()}>
      <Tabs
        containerClass={combineClasses(s.Sidebar, "shadow")}
        containerStyle={{
          width: inPx(SIDEBAR_WIDTH),
        }}
        tabsGap="0rem"
        tabs={[
          {
            title: "deployments",
            element: (
              <Grid
                class="scroller"
                style={{
                  height: "fit-content",
                  "max-height": inPx(height() - TOPBAR_HEIGHT - 80),
                  padding: "0rem 1rem",
                }}
              >
                <For each={servers.ids()}>{(id) => <Server id={id} />}</For>
                <Show when={permissions() >= 2}>
                  <AddServer />
                </Show>
              </Grid>
            ),
          },
          {
            title: "builds",
            element: (
              <Grid
                style={{
                  height: "fit-content",
                  "max-height": inPx(height() - TOPBAR_HEIGHT - 80),
                  padding: "0rem 1rem",
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
