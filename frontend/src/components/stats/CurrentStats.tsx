import { Params, useParams } from "@solidjs/router";
import ReconnectingWebSocket from "reconnecting-websocket";
import {
  Accessor,
  Component,
  createEffect,
  createSignal,
  For,
  JSXElement,
  Match,
  onCleanup,
  Setter,
  Show,
  Switch,
} from "solid-js";
import { client, URL } from "../..";
import { SystemStats } from "../../types";
import { generateQuery } from "../../util/helpers";
import { useLocalStorage } from "../../util/hooks";
import Circle from "../shared/Circle";
import HeatBar from "../shared/HeatBar";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Loading from "../shared/loading/Loading";
import HoverMenu from "../shared/menu/HoverMenu";
import SimpleTabs from "../shared/tabs/SimpleTabs";
import {
  CpuChart,
  DiskChart,
  DiskReadChart,
  DiskWriteChart,
  MemChart,
  NetworkRecvChart,
  NetworkSentChart,
  SingleTempuratureChart,
} from "./Charts";
import s from "./stats.module.scss";

const CurrentStats: Component<{}> = (p) => {
  const params = useParams();
  const [stats, setStats] = createSignal<SystemStats[]>([]);
  const { open } = useStatsWs(params, setStats);
  createEffect(() => {
    client
      .get_server_stats(params.id, {
        networks: true,
        components: true,
        processes: false,
      })
      .then((stats) => setStats([stats]));
  });
  const latest = () => stats()[stats().length - 1];
  return (
    <Grid class={s.Content} placeItems="start center">
      <Show when={stats().length > 0} fallback={<Loading type="three-dot" />}>
        <Grid class={s.HeatBars} placeItems="center start">
          <BasicInfo stats={stats} open={open} />
          <div />
          <SimpleTabs
            containerStyle={{ width: "100%", "min-width": "300px" }}
            localStorageKey={`${params.id}-io-tab-v1`}
            tabs={[
              {
                title: "network io",
                element: () => (
                  <Flex>
                    <NetworkRecvChart stats={stats} />
                    <NetworkSentChart stats={stats} />
                  </Flex>
                ),
              },
              {
                title: "disk io",
                element: () => (
                  <Flex>
                    <DiskReadChart stats={stats} />
                    <DiskWriteChart stats={stats} />
                  </Flex>
                ),
              },
            ]}
          />
          <div />

          <div />
          <h1>tempurature</h1>
          <div />

          <For
            each={latest().components?.filter((c) => c.critical !== undefined)}
          >
            {(comp) => (
              <StatsHeatbarRow
                type="temp"
                label={comp.label}
                stats={stats}
                percentage={(100 * comp.temp) / comp.critical!}
                localStorageKey={`${params.id}-temp-${comp.label}-v1`}
                additionalInfo={
                  <div style={{ opacity: 0.7 }}>{comp.temp.toFixed(1)}°</div>
                }
              />
            )}
          </For>
        </Grid>
      </Show>
    </Grid>
  );
};

export default CurrentStats;

const BasicInfo: Component<{
  stats: Accessor<SystemStats[]>;
  open: Accessor<boolean>;
}> = (p) => {
  const latest = () => p.stats()[p.stats().length - 1];
  const mem_perc = () => {
    return (100 * latest().mem_used_gb) / latest().mem_total_gb;
  };
  const disk_perc = () => {
    return (100 * latest().disk.used_gb) / latest().disk.total_gb;
  };
  return (
    <>
      <div />
      <Flex alignItems="center">
        <h1>basic</h1>
        <HoverMenu
          target={
            <Circle
              size={1}
              class={p.open() ? "green" : "red"}
              style={{ transition: "all 500ms ease-in-out" }}
            />
          }
          content={p.open() ? "connected" : "disconnected"}
          position="right center"
        />
      </Flex>
      <div />

      <StatsHeatbarRow
        label="cpu"
        type="cpu"
        stats={p.stats}
        percentage={latest().cpu_perc}
        localStorageKey="current-stats-cpu-graph-v1"
      />

      <StatsHeatbarRow
        label="mem"
        type="mem"
        stats={p.stats}
        percentage={mem_perc()}
        localStorageKey="current-stats-mem-graph-v1"
        additionalInfo={
          <div style={{ opacity: 0.7 }}>
            {latest().mem_used_gb.toFixed(1)} of{" "}
            {latest().mem_total_gb.toFixed()} GB
          </div>
        }
      />

      <StatsHeatbarRow
        label="disk"
        type="disk"
        stats={p.stats}
        percentage={disk_perc()}
        localStorageKey="current-stats-disk-graph-v1"
        additionalInfo={
          <div style={{ opacity: 0.7 }}>
            {latest().disk.used_gb.toFixed()} of{" "}
            {latest().disk.total_gb.toFixed()} GB
          </div>
        }
      />
    </>
  );
};

const StatsHeatbarRow: Component<{
  type: "cpu" | "mem" | "disk" | "temp";
  label: string;
  stats: Accessor<SystemStats[]>;
  percentage: number;
  localStorageKey: string;
  additionalInfo?: JSXElement;
}> = (p) => {
  const [showGraph, setShowGraph] = useLocalStorage(false, p.localStorageKey);
  return (
    <>
      <Show when={p.type === "temp"}>
        <div />
        <div>{p.label}</div>
        <div />
      </Show>
      <Show
        when={p.type !== "temp"}
        fallback={<div />}
      >
        <h1 style={{ "place-self": "center end" }}>{p.label}</h1>
      </Show>
      <HeatBar
        containerClass="card shadow"
        containerStyle={{ width: "60vw", "min-width": "300px" }}
        filled={Math.floor(p.percentage)}
        total={100}
        onClick={() => setShowGraph((curr) => !curr)}
      />
      <Grid gap="0">
        <h1>{p.percentage.toFixed(1)}%</h1>
        {p.additionalInfo}
      </Grid>
      <Show when={showGraph()}>
        <div />
        <Switch>
          <Match when={p.type === "cpu"}>
            <CpuChart stats={p.stats} small disableScroll />
          </Match>
          <Match when={p.type === "mem"}>
            <MemChart stats={p.stats} small disableScroll />
          </Match>
          <Match when={p.type === "disk"}>
            <DiskChart stats={p.stats} small disableScroll />
          </Match>
          <Match when={p.type === "temp"}>
            <SingleTempuratureChart
              component={p.label}
              stats={p.stats}
              small
              disableScroll
            />
          </Match>
        </Switch>
        <div />
      </Show>
    </>
  );
};

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
    console.log(stats);
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

// const NetworkIoInfo: Component<{ stats: Accessor<SystemStats[]> }> = (p) => {
//   const latest = () => p.stats()[p.stats().length - 1];
//   const network_recv = () => {
//     return latest().networks?.length || 0 > 0
//       ? latest()
//           .networks!.map((n) => n.recieved_kb)
//           .reduce((p, c) => p + c) /
//           get_to_one_sec_divisor(latest().polling_rate)!
//       : 0;
//   };
//   const network_sent = () => {
//     return latest().networks?.length || 0 > 0
//       ? latest()
//           .networks!.map((n) => n.transmitted_kb)
//           .reduce((p, c) => p + c) /
//           get_to_one_sec_divisor(latest().polling_rate)!
//       : 0;
//   };
//   return (
//     <>
//       <div />
//       <Flex alignItems="center">
//         <h1>network recv</h1>
//         <h2 style={{ opacity: 0.7 }}>{network_recv().toFixed(1)} kb/s</h2>
//       </Flex>
//       <div />

//       <div />
//       <NetworkRecvChart stats={p.stats} small disableScroll />
//       <div />

//       <div />
//       <Flex alignItems="center">
//         <h1>network sent</h1>
//         <h2 style={{ opacity: 0.7 }}>{network_sent().toFixed(1)} kb/s</h2>
//       </Flex>
//       <div />

//       <div />
//       <NetworkSentChart stats={p.stats} small disableScroll />
//       <div />
//     </>
//   );
// };

// const DiskIoInfo: Component<{ stats: Accessor<SystemStats[]> }> = (p) => {
//   const latest = () => p.stats()[p.stats().length - 1];
//   const disk_read = () => latest().disk.read_kb;
//   const disk_write = () => latest().disk.write_kb;
//   return (
//     <>
//       <div />
//       <Flex alignItems="center">
//         <h1>disk read</h1>
//         <h2 style={{ opacity: 0.7 }}>{disk_read().toFixed(1)} kb/s</h2>
//       </Flex>
//       <div />

//       <div />
//       <DiskReadChart stats={p.stats} small disableScroll />
//       <div />

//       <div />
//       <Flex alignItems="center">
//         <h1>disk write</h1>
//         <h2 style={{ opacity: 0.7 }}>{disk_write().toFixed(1)} kb/s</h2>
//       </Flex>
//       <div />

//       <div />
//       <DiskWriteChart stats={p.stats} small disableScroll />
//       <div />
//     </>
//   );
// };
