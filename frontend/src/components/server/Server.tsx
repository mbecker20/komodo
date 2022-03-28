import { Server as ServerType } from "@monitor/types";
import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Grid from "../util/layout/Grid";
import s from "./server.module.css";

const Server: Component<{}> = (p) => {
	const { servers, selected } = useAppState();
  const server = () => servers.get(selected.id()) as ServerType;
	return (
    <Show when={server()}>
      <Grid class={s.Server}>

			</Grid>
    </Show>
  );
}

export default Server;