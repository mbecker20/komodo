import { Component, For, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { combineClasses, inPx } from "../../util/helpers";
import AddServer from "../create/server";
import { TOPBAR_HEIGHT } from "../topbar/Topbar";
import Grid from "../util/layout/Grid";
import Server from "./server/Server";
import s from "./sidebar.module.css";

const SIDEBAR_WIDTH = 350;

const Sidebar: Component<{}> = (p) => {
  const { sidebar, servers } = useAppState();
  const { height } = useAppDimensions();
  return (
    <Show when={servers.loaded() && sidebar.open()}>
      <Grid
        class={combineClasses(s.Sidebar, "shadow")}
        style={{
          width: inPx(SIDEBAR_WIDTH),
          height: inPx(height() - TOPBAR_HEIGHT),
        }}
      >
        <Grid style={{ height: "fit-content", padding: "1rem" }}>
          <For each={servers.ids()}>{(id) => <Server id={id} />}</For>
          <AddServer />
        </Grid>
      </Grid>
    </Show>
  );
};

export default Sidebar;
