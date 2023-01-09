import { Params, useParams } from "@solidjs/router";
import ReconnectingWebSocket from "reconnecting-websocket";
import {
  Component,
  createEffect,
  createSignal,
  onCleanup,
  Setter,
  Show,
} from "solid-js";
import { client, URL } from "../..";
import { SystemStats } from "../../types";
import { combineClasses, generateQuery } from "../../util/helpers";
import HeatBar from "../shared/HeatBar";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Loading from "../shared/loading/Loading";
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
  const mem_perc = () => {
    return (100 * stats()!.mem_used_gb) / stats()!.mem_total_gb;
  };
  const disk_perc = () => {
    return (100 * stats()!.disk.used_gb) / stats()!.disk.total_gb;
  };
  return (
    <Grid class={s.Content} placeItems="start center">
      <Show when={stats()} fallback={<Loading type="three-dot" />}>
        <Grid class={s.HeatBars} placeItems="center start">
          <h1>cpu:</h1>
          <HeatBar
            containerClass="card shadow"
            containerStyle={{ width: "60vw", "min-width": "300px" }}
            filled={Math.floor(stats()!.cpu_perc)}
            total={100}
          />
          <h1>{stats()!.cpu_perc.toFixed(1)}%</h1>
          <h1>mem:</h1>
          <HeatBar
            containerClass="card shadow"
            containerStyle={{ width: "60vw", "min-width": "300px" }}
            filled={Math.floor(mem_perc())}
            total={100}
          />
          <Grid gap="0">
            <h1>{mem_perc().toFixed(1)}%</h1>
            <div style={{ opacity: 0.7 }}>
              {stats()!.mem_used_gb.toFixed()}GB of{" "}
              {stats()!.mem_total_gb.toFixed()}GB
            </div>
          </Grid>
          <h1>disk:</h1>
          <HeatBar
            containerClass="card shadow"
            containerStyle={{ width: "60vw", "min-width": "300px" }}
            filled={Math.floor(disk_perc())}
            total={100}
          />
          <Grid gap="0">
            <h1>{disk_perc().toFixed(1)}%</h1>
            <div style={{ opacity: 0.7 }}>
              {stats()!.disk.used_gb.toFixed()}GB of{" "}
              {stats()!.disk.total_gb.toFixed()}GB
            </div>
          </Grid>
        </Grid>
      </Show>
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
    // console.log(stats);
    setStats(stats);
  });
  ws.addEventListener("close", () => {
    console.log("stats connection closed");
    // clearInterval(int);
    setOpen(false);
  });
  onCleanup(() => {
    console.log("closing stats ws");
    ws.close();
  });
  return {
    open,
  };
}
