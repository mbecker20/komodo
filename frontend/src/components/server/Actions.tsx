import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import Grid from "../util/layout/Grid";
import s from "./server.module.css";

const Actions: Component<{}> = (p) => {
  const { ws, servers, selected } = useAppState();
  const server = () => servers.get(selected.id())!;
  return (
    <Show when={server() && server().status === "OK"}>
      <Grid class={combineClasses(s.Card, "shadow")}>
				<h1>actions</h1>
			</Grid>
    </Show>
  );
};

export default Actions;
