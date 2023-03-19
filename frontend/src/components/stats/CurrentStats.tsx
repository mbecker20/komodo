import { useParams } from "@solidjs/router";
import {
  Accessor,
  Component,
  createEffect,
  createSignal,
  For,
  JSXElement,
  Match,
  Show,
  Switch,
} from "solid-js";
import { client } from "../..";
import { SystemProcess, SystemStats } from "../../types";
import { convert_timelength_to_ms } from "../../util/helpers";
import { useLocalStorage } from "../../util/hooks";
import HeatBar from "../shared/HeatBar";
import Icon from "../shared/Icon";
import Input from "../shared/Input";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Loading from "../shared/loading/Loading";
import Selector from "../shared/menu/Selector";
import SimpleTabs from "../shared/tabs/SimpleTabs";
import {
  CpuChart,
  DiskChart,
  DiskReadChart,
  DiskWriteChart,
  LoadChart,
  MemChart,
  NetworkRecvChart,
  NetworkSentChart,
  SingleTempuratureChart,
} from "./Charts";
import { useStatsState } from "./Provider";
// import s from "./stats.module.scss";

function useCurrentStats() {
  const params = useParams();
  const { pollRate } = useStatsState();
  const [stats, setStats] = createSignal<SystemStats[]>([]);
  // useStatsWs(params, setStats, p.setWsOpen);
  const load = async () => {
    const stats = await client.get_server_stats(params.id, {
      networks: true,
      components: true,
      processes: true,
    });
    stats.refresh_ts = Date.now();
    setStats((curr) => [...(curr.length > 200 ? curr.slice(1) : curr), stats]);
  };
  let timeout = -1;
  let interval = -1;
  createEffect(() => {
    clearInterval(interval);
    clearTimeout(timeout);
    const pollRateMs = convert_timelength_to_ms(pollRate())!;
    timeout = setTimeout(() => {
      load();
      interval = setInterval(() => load(), pollRateMs);
    }, pollRateMs - (new Date().getTime() % pollRateMs));
  });
  load();
  const latest = () => stats()[stats().length - 1];

  return {
    params,
    stats,
    latest,
  };
}

const CurrentStats2: Component<{}> = (p) => {
  const { params, stats, latest } = useCurrentStats();
  return (
    <Grid
      style={{
        width: "100%",
        "box-sizing": "border-box",
      }}
      gridTemplateColumns="auto 1fr auto"
    >
      <Show when={stats().length > 0} fallback={<Loading type="three-dot" />}>
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
      </Show>

      <Processes stats={stats()} />
    </Grid>
  );
};

export default CurrentStats2;

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
        label="load"
        type="load"
        stats={p.stats}
        percentage={latest().system_load!}
        localStorageKey="current-stats-load-graph-v1"
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
        containerStyle={{
          width: "90vw",
          "max-width": "1050px",
          "place-self": "center",
          "box-sizing": "border-box",
        }}
        filled={Math.floor(p.percentage / 2)}
        total={50}
        onClick={() => setShowGraph((curr) => !curr)}
      />
      <Grid gap="0" style={{ "box-sizing": "border-box" }}>
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

const Processes: Component<{ stats: SystemStats[] }> = (p) => {
  const params = useParams();
  const [sortBy, setSortBy] = useLocalStorage(
    "cpu",
    `${params.id}-processes-sortby-v1`
  );
  const [filter, setFilter] = useLocalStorage(
    "",
    `${params.id}-processes-filter-v1`
  );
  const [showProcs, setShowProcs] = createSignal(true);
  return (
    <>
      <div />
      <Flex alignItems="center">
        <h1>processes</h1>
        <Selector
          label="sort by: "
          selected={sortBy()}
          items={["name", "cpu", "mem"]}
          onSelect={(item) => setSortBy(item)}
          position="bottom right"
          targetClass="grey"
        />
        <Flex alignItems="center">
          <Input
            placeholder="filter"
            value={filter()}
            onFocus={() => setShowProcs(false)}
            onConfirm={(filt) => {
              setShowProcs(true);
              setFilter(filt);
            }}
          />
          <Show when={filter().length > 0}>
            <button class="grey" onClick={() => setFilter("")}>
              <Icon type="cross" />
            </button>
          </Show>
        </Flex>
      </Flex>
      <div />

      <Show when={showProcs()}>
        <ProcessFor stats={p.stats} filter={filter()} sortBy={sortBy()} />
      </Show>
    </>
  );
};

const ProcessFor: Component<{
  filter: string;
  stats: SystemStats[];
  sortBy: string;
}> = (p) => {
  const latest = () => p.stats[p.stats.length - 1];
  const sort: () =>
    | ((a: SystemProcess, b: SystemProcess) => number)
    | undefined = () => {
    if (p.sortBy === "cpu") {
      return (a, b) => {
        return b.cpu_perc - a.cpu_perc;
      };
    } else if (p.sortBy === "mem") {
      return (a, b) => {
        return b.mem_mb - a.mem_mb;
      };
    } else if (p.sortBy === "name") {
      return (a, b) => {
        if (a.name > b.name) {
          return 1;
        } else {
          return -1;
        }
      };
    }
  };
  const processes = () => {
    const filters = p.filter.split(" ").filter((i) => i.length > 0);
    if (filters.length === 0) {
      return latest()?.processes?.sort(sort());
    }
    return latest()
      ?.processes?.filter((p) => {
        return filters.reduce((prev, curr) => {
          return prev || p.name.includes(curr);
        }, false);
      })
      .sort(sort());
  };
  return (
    <For each={processes()}>
      {(proc) => (
        <>
          <div />
          <Process proc={proc} />
          <div />
        </>
      )}
    </For>
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
        {/* <Flex gap="0.3rem" alignItems="center">
          <div>disk read:</div>
          <h2>{p.proc.disk_read_kb.toFixed(1)} kb</h2>
        </Flex>
        <Flex gap="0.3rem" alignItems="center">
          <div>disk write:</div>
          <h2>{p.proc.disk_write_kb.toFixed(1)} kb</h2>
        </Flex> */}
        <Flex gap="0.3rem" alignItems="center">
          <div>pid:</div>
          <h2>{p.proc.pid}</h2>
        </Flex>
      </Flex>
    </Flex>
  );
};
