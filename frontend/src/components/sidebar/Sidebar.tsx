import { Component, For, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { inPx } from "../../util/helpers";
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
        class={s.Sidebar}
        style={{
          width: inPx(SIDEBAR_WIDTH),
          height: inPx(height() - TOPBAR_HEIGHT),
        }}
      >
        <For each={servers.ids()}>{(id) => <Server id={id} />}</For>
      </Grid>
    </Show>
  );
};

export default Sidebar;
