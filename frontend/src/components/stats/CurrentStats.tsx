import { Params, useParams } from "@solidjs/router";
import ReconnectingWebSocket from "reconnecting-websocket";
import { Component, createEffect, createSignal, Setter } from "solid-js";
import { client, URL } from "../..";
import { SystemStats } from "../../types";
import { generateQuery } from "../../util/helpers";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import s from "./stats.module.scss";

const CurrentStats: Component<{}> = (p) => {
  const params = useParams();
  const [stats, setStats] = createSignal<SystemStats>();
  const open = useStatsWs(params, setStats);
  createEffect(() => {
    client
      .get_server_stats(params.id, {
        networks: true,
        components: true,
        processes: true,
      })
      .then(setStats);
  });
  return (
    <Grid class={s.Content}>
      <Flex>
        <div>cpu:</div>
				<h2>{}</h2>
      </Flex>
    </Grid>
  );
};

export default CurrentStats;

function useStatsWs(params: Params, setStats: Setter<SystemStats>) {
  const ws = new ReconnectingWebSocket(
    `${URL.replace("http", "ws")}/ws/stats/${params.id}${generateQuery({
      networks: "true",
      components: "true",
      processes: "true",
    })}`
  );
  const [open, setOpen] = createSignal(false);
  ws.addEventListener("open", () => {
    // console.log("connection opened");
    ws.send(client.token!);
    setOpen(true);
  });
  ws.addEventListener("message", ({ data }) => {
    if (data === "LOGGED_IN") {
      console.log("logged in to ws");
      return;
    }
    const stats = JSON.parse(data) as SystemStats;
    console.log(stats);
    setStats(stats);
  });
  ws.addEventListener("close", () => {
    console.log("stats connection closed");
    // clearInterval(int);
    setOpen(false);
  });
  return {
    open,
  };
}
