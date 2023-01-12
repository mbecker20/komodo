import { Accessor, Component, For, Show } from "solid-js";
import { SystemStats, SystemStatsRecord } from "../../types";
import {
  convertTsMsToLocalUnixTsInSecs,
  get_to_one_sec_divisor,
} from "../../util/helpers";
import Grid from "../shared/layout/Grid";
import LightweightChart, { LineDataPoint } from "../shared/LightweightChart";
import s from "./stats.module.scss";

export const COLORS = {
  blue: "#184e9f",
  orange: "#ac5c36",
  purple: "#5A0B4D",
  green: "#41764c",
  red: "#952E23",
};

const CHART_HEIGHT = "250px";
const SMALL_CHART_HEIGHT = "150px";

const SingleStatChart: Component<{
  line: () => LineDataPoint[] | undefined;
  header: string;
  label: string;
  color: string;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  return (
    <Show when={p.line()}>
      <Grid
        gap="0.5rem"
        class="card shadow"
        style={{
          height: "fit-content",
          width: "100%",
          "box-sizing": "border-box",
          "padding-top": "0.5rem",
          "padding-bottom": "0.2rem",
        }}
      >
        <Show when={!p.small} fallback={<div>{p.header}</div>}>
          <h2>{p.header}</h2>
        </Show>
        <LightweightChart
          class={s.LightweightChart}
          height={p.small ? SMALL_CHART_HEIGHT : CHART_HEIGHT}
          lines={() => [{ title: p.label, color: p.color, line: p.line()! }]}
          disableScroll={p.disableScroll}
        />
      </Grid>
    </Show>
  );
};

export const LoadChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const line = () => {
    return p.stats()?.filter(s => s.system_load !== undefined).map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: s.system_load!,
      };
    });
  };
  return (
    <SingleStatChart
      header="system load %"
      label="load %"
      color={COLORS.blue}
      line={line}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

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
    <SingleStatChart
      header="cpu %"
      label="cpu %"
      color={COLORS.blue}
      line={line}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const CpuFreqChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const line = () => {
    return p
      .stats()
      ?.filter((s) => s.cpu_freq_mhz !== undefined)
      .map((s) => {
        return {
          time: convertTsMsToLocalUnixTsInSecs(
            (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
          ),
          value: s.cpu_freq_mhz! / 1000,
        };
      });
  };
  return (
    <SingleStatChart
      header="cpu frequency"
      label="GHz"
      color={COLORS.blue}
      line={line}
      small={p.small}
      disableScroll={p.disableScroll}
    />
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
    <SingleStatChart
      header="memory"
      label="mem %"
      color={COLORS.green}
      line={line}
      small={p.small}
      disableScroll={p.disableScroll}
    />
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
    <SingleStatChart
      header="disk"
      label="disk %"
      color={COLORS.orange}
      line={line}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const NetworkRecvChart: Component<{
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
  return (
    <SingleStatChart
      header="network received kb/s"
      label="recv kb/s"
      color={COLORS.green}
      line={recv_line}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const NetworkSentChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const sent_line = () => {
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
    <SingleStatChart
      header="network sent kb/s"
      label="sent kb/s"
      color={COLORS.red}
      line={sent_line}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const DiskReadChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
  gridFillers?: boolean;
}> = (p) => {
  const read_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: s.disk.read_kb / get_to_one_sec_divisor(s.polling_rate)!,
      };
    });
  };
  return (
    <SingleStatChart
      header="disk read kb/s"
      label="read kb/s"
      color={COLORS.green}
      line={read_line}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const DiskWriteChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
  gridFillers?: boolean;
}> = (p) => {
  const write_line = () => {
    return p.stats()?.map((s) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: s.disk.write_kb / get_to_one_sec_divisor(s.polling_rate)!,
      };
    });
  };
  return (
    <SingleStatChart
      header="disk write kb/s"
      label="write kb/s"
      color={COLORS.red}
      line={write_line}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};

export const TempuratureChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const labels = () => {
    return p.stats()![p.stats()!.length - 1].components!.map((c) => c.label);
  };
  return (
    <For each={labels()}>
      {(component) => (
        <SingleTempuratureChart
          component={component}
          stats={p.stats}
          small={p.small}
          disableScroll={p.disableScroll}
        />
      )}
    </For>
  );
};

export const SingleTempuratureChart: Component<{
  stats: Accessor<(SystemStatsRecord | SystemStats)[] | undefined>;
  component: string;
  small?: boolean;
  disableScroll?: boolean;
}> = (p) => {
  const line = () => {
    return p.stats()?.map((s) => {
      const temp = s.components!.find((c) => c.label === p.component)?.temp;
      return {
        time: convertTsMsToLocalUnixTsInSecs(
          (s as SystemStatsRecord).ts || (s as SystemStats).refresh_ts
        ),
        value: temp || 0,
      };
    });
  };
  return (
    <SingleStatChart
      header={p.component}
      label="temp"
      color={COLORS.red}
      line={line}
      small={p.small}
      disableScroll={p.disableScroll}
    />
  );
};
