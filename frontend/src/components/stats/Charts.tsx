import { Accessor, Component, createSignal, For, Show } from "solid-js";
import { SystemStats, SystemStatsRecord } from "../../types";
import {
  convertTsMsToLocalUnixTsInSecs,
  get_to_one_sec_divisor,
} from "../../util/helpers";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import LightweightChart from "../shared/LightweightChart";
import s from "./stats.module.scss";

export const COLORS = {
  blue: "#184e9f",
  orange: "#ac5c36",
  purple: "#5A0B4D",
  green: "#41764c",
  red: "#952E23",
};

const CHART_HEIGHT = "200px";
const SMALL_CHART_HEIGHT = "150px";

export const CpuChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: s.cpu_perc,
      };
    });
  };
  return (
    <Show when={line()}>
      <Grid
        gap="0"
        class="card shadow"
        style={{
          height: "fit-content",
          width: "100%",
          "box-sizing": "border-box",
          "padding-bottom": "0.2rem",
        }}
      >
        <Show when={!p.small}>
          <h2>cpu</h2>
        </Show>
        <LightweightChart
          class={s.LightweightChart}
          height={p.small ? SMALL_CHART_HEIGHT : CHART_HEIGHT}
          lines={() => [{ title: "%", color: COLORS.blue, line: line()! }]}
          disableScroll={p.disableScroll}
        />
      </Grid>
    </Show>
  );
};

export const MemChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: (100 * s.mem_used_gb) / s.mem_total_gb,
      };
    });
  };
  return (
    <Show when={line()}>
      <Grid
        gap="0"
        class="card shadow"
        style={{
          height: "fit-content",
          width: "100%",
          "box-sizing": "border-box",
          "padding-bottom": "0.2rem",
        }}
      >
        <Show when={!p.small}>
          <h2>memory</h2>
        </Show>
        <LightweightChart
          class={s.LightweightChart}
          height={p.small ? SMALL_CHART_HEIGHT : CHART_HEIGHT}
          lines={() => [{ title: "mem %", color: COLORS.blue, line: line()! }]}
          disableScroll={p.disableScroll}
        />
      </Grid>
    </Show>
  );
};

export const DiskChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: (100 * s.disk.used_gb) / s.disk.total_gb,
      };
    });
  };
  return (
    <Show when={line()}>
      <Grid
        gap="0"
        class="card shadow"
        style={{
          height: "fit-content",
          width: "100%",
          "box-sizing": "border-box",
          "padding-bottom": "0.2rem",
        }}
      >
        <Show when={!p.small}>
          <h2>disk</h2>
        </Show>
        <LightweightChart
          class={s.LightweightChart}
          height={p.small ? SMALL_CHART_HEIGHT : CHART_HEIGHT}
          lines={() => [{ title: "disk %", color: "#184e9f", line: line()! }]}
          disableScroll={p.disableScroll}
        />
      </Grid>
    </Show>
  );
};

export const NetworkIoCharts: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const recv_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
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
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
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
      <Grid
        gap="0"
        class="card shadow"
        style={{
          height: "fit-content",
          width: "100%",
          "box-sizing": "border-box",
          "padding-bottom": "0.2rem",
        }}
      >
        <Show when={!p.small}>
          <h2>network sent kb/s</h2>
        </Show>
        <LightweightChart
          class={s.LightweightChart}
          height={p.small ? SMALL_CHART_HEIGHT : CHART_HEIGHT}
          lines={() => [
            { title: "sent kb/s", color: "#184e9f", line: trans_line()! },
          ]}
        />
      </Grid>
      <Grid
        gap="0"
        class="card shadow"
        style={{
          height: "fit-content",
          width: "100%",
          "box-sizing": "border-box",
          "padding-bottom": "0.2rem",
        }}
      >
        <Show when={!p.small}>
          <h2>network received kb/s</h2>
        </Show>
        <LightweightChart
          class={s.LightweightChart}
          height={p.small ? SMALL_CHART_HEIGHT : CHART_HEIGHT}
          lines={() => [
            { title: "recv kb/s", color: "#184e9f", line: recv_line()! },
          ]}
          disableScroll={p.disableScroll}
        />
      </Grid>
    </Show>
  );
};

export const DiskIoCharts: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const read_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value:
          s.disk.disks?.length || 0 > 0
            ? s.disk.read_kb / get_to_one_sec_divisor(s.polling_rate)!
            : 0,
      };
    });
  };
  const write_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value:
          s.disk.disks?.length || 0 > 0
            ? s.disk.write_kb / get_to_one_sec_divisor(s.polling_rate)!
            : 0,
      };
    });
  };
  return (
    <Show when={read_line()}>
      <Grid
        gap="0"
        class="card shadow"
        style={{
          height: "fit-content",
          width: "100%",
          "box-sizing": "border-box",
          "padding-bottom": "0.2rem",
        }}
      >
        <Show when={!p.small}>
          <h2>disk read kb/s</h2>
        </Show>
        <LightweightChart
          class={s.LightweightChart}
          height={p.small ? SMALL_CHART_HEIGHT : CHART_HEIGHT}
          lines={() => [
            { title: "kb/s", color: "#184e9f", line: read_line()! },
          ]}
        />
      </Grid>
      <Grid
        gap="0"
        class="card shadow"
        style={{
          height: "fit-content",
          width: "100%",
          "box-sizing": "border-box",
          "padding-bottom": "0.2rem",
        }}
      >
        <h2>disk write kb/s</h2>
        <LightweightChart
          class={s.LightweightChart}
          height={p.small ? SMALL_CHART_HEIGHT : CHART_HEIGHT}
          lines={() => [
            { title: "kb/s", color: "#184e9f", line: write_line()! },
          ]}
          disableScroll={p.disableScroll}
        />
      </Grid>
    </Show>
  );
};

export const TempuratureChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  // const [selected, setSelected] = createSignal(p.stats()![p.stats()!.length - 1].components![0].label);
  const labels = () => {
    return p.stats()![p.stats()!.length - 1].components!.map((c) => c.label);
  };
  const line = (component: string) => {
    return p.stats()?.map((s) => {
      const temp = s.components!.find((c) => c.label === component)?.temp;
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
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
          style={{
            height: "fit-content",
            width: "100%",
            "box-sizing": "border-box",
            "padding-bottom": "0.2rem",
          }}
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
            height={p.small ? SMALL_CHART_HEIGHT : CHART_HEIGHT}
            lines={() => [
              { title: "temp", color: "#184e9f", line: line(label)! },
            ]}
            disableScroll={p.disableScroll}
          />
        </Grid>
      )}
    </For>
  );
};
