import { Params, useParams } from "@solidjs/router";
import { ISeriesApi } from "lightweight-charts";
import ReconnectingWebSocket from "reconnecting-websocket";
import {
  Component,
  createEffect,
  createSignal,
  onCleanup,
  Setter,
  Show,
} from "solid-js";
import { createStore } from "solid-js/store";
import { client, URL } from "../..";
import { SystemStats } from "../../types";
import { generateQuery } from "../../util/helpers";
import { useLocalStorage } from "../../util/hooks";
import HeatBar from "../shared/HeatBar";
import Grid from "../shared/layout/Grid";
import Loading from "../shared/loading/Loading";
import { CpuChart, DiskChart, MemChart } from "./Charts";
import s from "./stats.module.scss";

const CurrentStats: Component<{}> = (p) => {
  const params = useParams();
  const [showCpuGraph, setShowCpuGraph] = useLocalStorage(
    false,
    "current-stats-cpu-graph-v1"
  );
  const [showMemGraph, setShowMemGraph] = useLocalStorage(
    false,
    "current-stats-mem-graph-v1"
  );
  const [showDiskGraph, setShowDiskGraph] = useLocalStorage(
    false,
    "current-stats-disk-graph-v1"
  );
  const [stats, setStats] = createSignal<SystemStats[]>([]);
  const open = useStatsWs(params, setStats);
  createEffect(() => {
    client
      .get_server_stats(params.id, {
        networks: true,
        components: true,
        processes: true,
      })
      .then((stats) => setStats([stats]));
  });
  const mem_perc = () => {
    return (
      (100 * stats()[stats()!.length - 1].mem_used_gb) /
      stats()![stats().length - 1].mem_total_gb
    );
  };
  const disk_perc = () => {
    return (
      (100 * stats()[stats().length - 1].disk.used_gb) /
      stats()![stats().length - 1].disk.total_gb
    );
  };
  return (
    <Grid class={s.Content} placeItems="start center">
      <Show when={stats().length > 0} fallback={<Loading type="three-dot" />}>
        <Grid class={s.HeatBars} placeItems="center start">
          <div />
          <h1>basic</h1>
          <div />

          <h1>cpu:</h1>
          <HeatBar
            containerClass="card shadow"
            containerStyle={{ width: "60vw", "min-width": "300px" }}
            filled={Math.floor(stats()[stats().length - 1].cpu_perc)}
            total={100}
            onClick={() => setShowCpuGraph((curr) => !curr)}
          />
          <h1>{stats()[stats().length - 1].cpu_perc.toFixed(1)}%</h1>
          <Show when={showCpuGraph()}>
            <div />
            <CpuChart stats={stats} small disableScroll />
            <div />
          </Show>
          <h1>mem:</h1>
          <HeatBar
            containerClass="card shadow"
            containerStyle={{ width: "60vw", "min-width": "300px" }}
            filled={Math.floor(mem_perc())}
            total={100}
            onClick={() => setShowMemGraph((curr) => !curr)}
          />
          <Grid gap="0">
            <h1>{mem_perc().toFixed(1)}%</h1>
            <div style={{ opacity: 0.7 }}>
              {stats()[stats().length - 1].mem_used_gb.toFixed()}GB of{" "}
              {stats()[stats().length - 1].mem_total_gb.toFixed()}GB
            </div>
          </Grid>
          <Show when={showMemGraph()}>
            <div />
            <MemChart stats={stats} small disableScroll />
            <div />
          </Show>
          <h1>disk:</h1>
          <HeatBar
            containerClass="card shadow"
            containerStyle={{ width: "60vw", "min-width": "300px" }}
            filled={Math.floor(disk_perc())}
            total={100}
            onClick={() => setShowDiskGraph(curr => !curr)}
          />
          <Grid gap="0">
            <h1>{disk_perc().toFixed(1)}%</h1>
            <div style={{ opacity: 0.7 }}>
              {stats()[stats().length - 1].disk.used_gb.toFixed()}GB of{" "}
              {stats()[stats().length - 1].disk.total_gb.toFixed()}GB
            </div>
          </Grid>
          <Show when={showDiskGraph()}>
            <div />
            <DiskChart stats={stats} small disableScroll />
            <div />
          </Show>
        </Grid>
      </Show>
    </Grid>
  );
};

export default CurrentStats;

function useStatsWs(params: Params, setStats: Setter<SystemStats[]>) {
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
    setStats((stats_arr) => [
      ...(stats_arr.length > 200 ? stats_arr.slice(1) : stats_arr),
      stats,
    ]);
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
