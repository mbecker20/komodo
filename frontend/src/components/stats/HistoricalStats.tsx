import { useParams } from "@solidjs/router";
import { Accessor, Component, createEffect, createSignal, For, Match, Show, Switch } from "solid-js";
import { client } from "../..";
import { SystemStatsRecord, Timelength } from "../../types";
import { convertTsMsToLocalUnixTsInSecs, get_to_one_sec_divisor } from "../../util/helpers";
import { useLocalStorage } from "../../util/hooks";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import LightweightChart from "../shared/LightweightChart";
import Loading from "../shared/loading/Loading";
import Selector from "../shared/menu/Selector";
import s from "./stats.module.scss";

const TIMELENGTHS = [
  Timelength.OneMinute,
  Timelength.FiveMinutes,
  Timelength.FifteenMinutes,
  Timelength.OneHour,
  Timelength.SixHours,
  Timelength.TwelveHours,
  Timelength.OneDay,
];

const COLORS = {
  blue: "#184e9f",
  orange: "#ac5c36",
  purple: "#5A0B4D",
  green: "#41764c",
  red: "#952E23",
};

const VIEWS = [
  "basic",
  "i / o",
  "temp"
];

const HistoricalStats: Component<{}> = (p) => {
  const params = useParams();
  const [timelength, setTimelength] = useLocalStorage(
    Timelength.OneMinute,
    "server-stats-timelength-v3"
  );
  const [view, setView] = useLocalStorage("basic", "historical-stats-view-v1")
  const [stats, setStats] = createSignal<SystemStatsRecord[]>();
  const [page, setPage] = createSignal(0);
  createEffect(() => {
    client
      .get_server_stats_history(params.id, {
        interval: timelength(),
        page: page(),
        limit: 1000,
        networks: true,
        components: true,
      })
      .then(setStats);
  });
  return (
    <Grid class={s.Content}>
      <Flex alignItems="center">
        <Flex class="card light shadow" alignItems="center">
          <button
            class="darkgrey"
            onClick={() => {
              setPage((page) => page + 1);
            }}
          >
            <Icon type="chevron-left" />
          </button>
          <button
            class="darkgrey"
            onClick={() => {
              setPage((page) => (page > 0 ? page - 1 : 0));
            }}
          >
            <Icon type="chevron-right" />
          </button>
          <button
            class="darkgrey"
            onClick={() => {
              setPage(0);
            }}
          >
            <Icon type="double-chevron-right" />
          </button>
          <div>page: {page() + 1}</div>
        </Flex>
        <Selector
          targetClass="grey"
          selected={timelength()}
          items={TIMELENGTHS}
          onSelect={(selected) => {
            setPage(0);
            setTimelength(selected as Timelength);
          }}
        />
        <Selector
          targetClass="grey"
          selected={view()}
          items={VIEWS}
          onSelect={(selected) => {
            setView(selected);
          }}
        />
      </Flex>
      <Show when={stats()} fallback={<Loading type="three-dot" />}>
        <Switch>
          <Match when={view() === "basic"}>
            <Grid class={s.Charts}>
              <CpuChart stats={stats} />
              <MemChart stats={stats} />
              <DiskChart stats={stats} />
            </Grid>
          </Match>
          <Match when={view() === "i / o"}>
            <Grid class={s.Charts}>
              <DiskIoCharts stats={stats} />
              <NetworkIoCharts stats={stats} />
            </Grid>
          </Match>
          <Match when={view() === "temp"}>
            <Grid class={s.Charts}>
              <TempuratureChart stats={stats} />
            </Grid>
          </Match>
        </Switch>
      </Show>
    </Grid>
  );
};

const CpuChart: Component<{
  stats: Accessor<SystemStatsRecord[] | undefined>;
}> = (p) => {
  const line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(s.ts),
        value: s.cpu_perc,
      };
    });
  };
  return (
    <Show when={line()}>
      <Grid gap="0" class="card shadow" style={{ height: "fit-content" }}>
        <h2>cpu</h2>
        <LightweightChart
          class={s.LightweightChart}
          style={{ height: "200px" }}
          lines={() => [{ title: "%", color: COLORS.blue, line: line()! }]}
        />
      </Grid>
    </Show>
  );
};

const MemChart: Component<{
  stats: Accessor<SystemStatsRecord[] | undefined>;
}> = (p) => {
  const [selected, setSelected] = createSignal("%");
  const line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(s.ts),
        value:
          selected() === "%"
            ? (100 * s.mem_used_gb) / s.mem_total_gb
            : s.mem_used_gb,
      };
    });
  };
  return (
    <Show when={line()}>
      <Grid gap="0" class="card shadow" style={{ height: "fit-content" }}>
        <Flex alignItems="center" justifyContent="space-between">
          <h2>memory</h2>
          {/* <Selector
            selected={selected()}
            items={["%", "GB"]}
            onSelect={setSelected}
          /> */}
        </Flex>
        <LightweightChart
          class={s.LightweightChart}
          style={{ height: "200px" }}
          lines={() => [{ title: selected(), color: COLORS.blue, line: line()! }]}
        />
      </Grid>
    </Show>
  );
};

const DiskChart: Component<{
  stats: Accessor<SystemStatsRecord[] | undefined>;
}> = (p) => {
  const [selected, setSelected] = createSignal("%");
  const line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(s.ts),
        value:
          selected() === "%"
            ? (100 * s.disk.used_gb) / s.disk.total_gb
            : s.disk.used_gb,
      };
    });
  };
  return (
    <Show when={line()}>
      <Grid gap="0" class="card shadow" style={{ height: "fit-content" }}>
        <Flex alignItems="center" justifyContent="space-between">
          <h2>disk</h2>
          {/* <Selector
            selected={selected()}
            items={["%", "GB"]}
            onSelect={setSelected}
          /> */}
        </Flex>
        <LightweightChart
          class={s.LightweightChart}
          style={{ height: "200px" }}
          lines={() => [{ title: selected(), color: "#184e9f", line: line()! }]}
        />
      </Grid>
    </Show>
  );
};

const NetworkIoCharts: Component<{
  stats: Accessor<SystemStatsRecord[] | undefined>;
}> = (p) => {
  const recv_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(s.ts),
        value:
          s.networks?.length || 0 > 0
            ? s.networks!.map((n) => n.recieved_kb).reduce((p, c) => p + c) /
              get_to_one_sec_divisor(s.polling_rate)!
            : 0,
      };
    });
  };
  const trans_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(s.ts),
        value:
          s.networks?.length || 0 > 0
            ? s.networks!.map((n) => n.transmitted_kb).reduce((p, c) => p + c) /
              get_to_one_sec_divisor(s.polling_rate)!
            : 0,
      };
    });
  };
  return (
    <Show when={recv_line()}>
      <Grid gap="0" class="card shadow" style={{ height: "fit-content" }}>
        <Flex alignItems="center" justifyContent="space-between">
          <h2>network sent kb/s</h2>
        </Flex>
        <LightweightChart
          class={s.LightweightChart}
          style={{ height: "200px" }}
          lines={() => [
            { title: "kb/s", color: "#184e9f", line: trans_line()! },
          ]}
        />
      </Grid>
      <Grid gap="0" class="card shadow" style={{ height: "fit-content" }}>
        <Flex alignItems="center" justifyContent="space-between">
          <h2>network received kb/s</h2>
        </Flex>
        <LightweightChart
          class={s.LightweightChart}
          style={{ height: "200px" }}
          lines={() => [
            { title: "kb/s", color: "#184e9f", line: recv_line()! },
          ]}
        />
      </Grid>
    </Show>
  );
};

const DiskIoCharts: Component<{
  stats: Accessor<SystemStatsRecord[] | undefined>;
}> = (p) => {
  const read_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(s.ts),
        value:
          s.disk.disks?.length || 0 > 0
            ? s.disk.read_kb /
              get_to_one_sec_divisor(s.polling_rate)!
            : 0,
      };
    });
  };
  const write_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(s.ts),
        value:
          s.disk.disks?.length || 0 > 0
            ? s.disk.write_kb / get_to_one_sec_divisor(s.polling_rate)!
            : 0,
      };
    });
  };
  return (
    <Show when={read_line()}>
      <Grid gap="0" class="card shadow" style={{ height: "fit-content" }}>
        <Flex alignItems="center" justifyContent="space-between">
          <h2>disk read kb/s</h2>
        </Flex>
        <LightweightChart
          class={s.LightweightChart}
          style={{ height: "200px" }}
          lines={() => [
            { title: "kb/s", color: "#184e9f", line: read_line()! },
          ]}
        />
      </Grid>
      <Grid gap="0" class="card shadow" style={{ height: "fit-content" }}>
        <Flex alignItems="center" justifyContent="space-between">
          <h2>disk write kb/s</h2>
        </Flex>
        <LightweightChart
          class={s.LightweightChart}
          style={{ height: "200px" }}
          lines={() => [
            { title: "kb/s", color: "#184e9f", line: write_line()! },
          ]}
        />
      </Grid>
    </Show>
  );
};

const TempuratureChart: Component<{
  stats: Accessor<SystemStatsRecord[] | undefined>;
}> = (p) => {
  // const [selected, setSelected] = createSignal(p.stats()![p.stats()!.length - 1].components![0].label);
  const labels = () => {
    return p.stats()![p.stats()!.length - 1].components!.map((c) => c.label);
  };
  const line = (component: string) => {
    return p.stats()?.map((s) => {
      const temp = s.components!.find((c) => c.label === component)?.temp;
      return {
        time: convertTsMsToLocalUnixTsInSecs(s.ts),
        value: temp || 0,
      };
    });
  };
  return (
    <For each={labels()}>
      {(label) => (
        <Grid
          gap="0"
          class="card shadow"
          style={{ height: "fit-content" }}
        >
          <Flex alignItems="center" justifyContent="space-between">
            <h2>{label}</h2>
            {/* <Selector
          selected={selected()}
          items={labels()}
          onSelect={setSelected}
        /> */}
          </Flex>
          <LightweightChart
            class={s.LightweightChart}
            style={{ height: "200px" }}
            lines={() => [
              { title: "temp", color: "#184e9f", line: line(label)! },
            ]}
          />
        </Grid>
      )}
    </For>
  );
};

export default HistoricalStats;
