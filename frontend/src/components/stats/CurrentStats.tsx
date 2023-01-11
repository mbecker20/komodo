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
import { SystemProcess, SystemStats } from "../../types";
import { generateQuery } from "../../util/helpers";
import { useLocalStorage } from "../../util/hooks";
import HeatBar from "../shared/HeatBar";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Loading from "../shared/loading/Loading";
import SimpleTabs from "../shared/tabs/SimpleTabs";
import {
  CpuChart,
  CpuFreqChart,
  DiskChart,
  DiskReadChart,
  DiskWriteChart,
  LoadChart,
  MemChart,
  NetworkRecvChart,
  NetworkSentChart,
  SingleTempuratureChart,
} from "./Charts";
import s from "./stats.module.scss";

const CurrentStats: Component<{ setWsOpen: Setter<boolean> }> = (p) => {
  const params = useParams();
  const [stats, setStats] = createSignal<SystemStats[]>([]);
  useStatsWs(params, setStats, p.setWsOpen);
  createEffect(() => {
    client
      .get_server_stats(params.id, {
        networks: true,
        components: true,
        processes: true,
      })
      .then((stats) => setStats([stats]));
  });
  const latest = () => stats()[stats().length - 1];
  return (
    <Grid class={s.Content} placeItems="start center">
      <Show when={stats().length > 0} fallback={<Loading type="three-dot" />}>
        <Grid class={s.HeatBars} placeItems="center start">
          <BasicInfo stats={stats} />
          <div />
          <SimpleTabs
            containerStyle={{ width: "100%", "min-width": "300px" }}
            localStorageKey={`${params.id}-io-tab-v1`}
            tabs={[
              {
                title: "network io",
                element: () => (
                  <Flex>
                    <NetworkRecvChart stats={stats} small disableScroll />
                    <NetworkSentChart stats={stats} small disableScroll />
                  </Flex>
                ),
              },
              {
                title: "disk io",
                element: () => (
                  <Flex>
                    <DiskReadChart stats={stats} small disableScroll />
                    <DiskWriteChart stats={stats} small disableScroll />
                  </Flex>
                ),
              },
            ]}
          />
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

          <div />
          <h1>processes</h1>
          <div />

          <For
            each={latest().processes?.filter(
              (p) => p.cpu_perc > 0 || p.mem_mb > 0
            )}
          >
            {(proc) => (
              <>
                <div />
                <Process proc={proc} />
                <div />
              </>
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
      <StatsHeatbarRow
        label="load"
        type="load"
        stats={p.stats}
        percentage={latest().system_load!}
        localStorageKey="current-stats-load-graph-v1"
      />

      <StatsHeatbarRow
        label="cpu"
        type="cpu"
        stats={p.stats}
        percentage={latest().cpu_perc}
        localStorageKey="current-stats-cpu-graph-v1"
        additionalInfo={
          <div style={{ opacity: 0.7 }}>
            {(latest().cpu_freq_mhz / 1000).toFixed(1)} GHz
          </div>
        }
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
  type: "cpu" | "load" | "mem" | "disk" | "temp";
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
        <h2>{p.label}</h2>
        <div />
      </Show>
      <Show when={p.type !== "temp"} fallback={<div />}>
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
          <Match when={p.type === "load"}>
            <LoadChart stats={p.stats} small disableScroll />
          </Match>
          <Match when={p.type === "cpu"}>
            <Flex style={{ width: "100%" }}>
              <CpuChart stats={p.stats} small disableScroll />
              <CpuFreqChart stats={p.stats} small disableScroll />
            </Flex>
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

const Process: Component<{ proc: SystemProcess }> = (p) => {
  return (
    <Flex
      class="card shadow"
      alignItems="center"
      justifyContent="space-between"
      style={{ width: "100%", "box-sizing": "border-box" }}
    >
      <h2>{p.proc.name}</h2>
      <Flex alignItems="center">
        <Flex gap="0.3rem" alignItems="center">
          <div>cpu:</div>
          <h2>{p.proc.cpu_perc.toFixed(1)}%</h2>
        </Flex>
        <Flex gap="0.3rem" alignItems="center">
          <div>mem:</div>
          <h2>{p.proc.mem_mb.toFixed(1)} mb</h2>
        </Flex>
        <Flex gap="0.3rem" alignItems="center">
          <div>disk read:</div>
          <h2>{p.proc.disk_read_kb.toFixed(1)} kb</h2>
        </Flex>
        <Flex gap="0.3rem" alignItems="center">
          <div>disk write:</div>
          <h2>{p.proc.disk_write_kb.toFixed(1)} kb</h2>
        </Flex>
        <Flex gap="0.3rem" alignItems="center">
          <div>pid:</div>
          <h2>{p.proc.pid}</h2>
        </Flex>
      </Flex>
    </Flex>
  );
};

function useStatsWs(params: Params, setStats: Setter<SystemStats[]>, setWsOpen: Setter<boolean>) {
  const ws = new ReconnectingWebSocket(
    `${URL.replace("http", "ws")}/ws/stats/${params.id}${generateQuery({
      networks: "true",
      components: "true",
      processes: "true",
      cpus: "true",
    })}`
  );
  ws.addEventListener("open", () => {
    // console.log("connection opened");
    ws.send(client.token!);
    setWsOpen(true);
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
    setWsOpen(false);
  });
  onCleanup(() => {
    console.log("closing stats ws");
    ws.close();
  });
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
