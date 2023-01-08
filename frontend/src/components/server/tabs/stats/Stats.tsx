import { useParams } from "@solidjs/router";
import {
  Accessor,
  Component,
  createEffect,
  createMemo,
  createSignal,
  Show,
} from "solid-js";
import { client } from "../../../..";
import { SystemStats, SystemStatsRecord, Timelength } from "../../../../types";
import { convertTsMsToLocalUnixTsInSecs } from "../../../../util/helpers";
import { useLocalStorage } from "../../../../util/hooks";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import LightweightChart from "../../../shared/LightweightChart";
import Loading from "../../../shared/loading/Loading";
import Selector from "../../../shared/menu/Selector";
import s from "./stats.module.scss";

const TIMELENGTHS = [
  Timelength.OneMinute,
  Timelength.FiveMinutes,
  Timelength.FifteenMinutes,
  Timelength.OneHour,
  Timelength.SixHours,
  Timelength.OneDay,
];

const Stats: Component<{}> = (p) => {
  const params = useParams();
  const [timelength, setTimelength] = useLocalStorage(
    Timelength.OneMinute,
    "server-stats-timelength-v3"
  );
  const [currStats, setCurrStats] = createSignal<SystemStats>();
  const [stats, setStats] = createSignal<SystemStatsRecord[]>();
  createEffect(() => {
    client.get_server_stats(params.id).then(setCurrStats);
    client
      .get_server_stats_history(params.id, {
        interval: timelength(),
        networks: true,
        components: true,
      })
      .then(setStats);
  });
  // createEffect(() => console.log(stats()))
  return (
    <Grid
      style={{
        width: "100%",
        height: "fit-content",
        padding: "1rem 3rem",
        "box-sizing": "border-box",
      }}
    >
      <Flex
        style={{ width: "100%" }}
        alignItems="center"
        justifyContent="space-between"
      >
        <Flex class="card light shadow" alignItems="center">
          <Show when={currStats()} fallback={<Loading type="three-dot" />}>
            <Grid gap="0" placeItems="start center">
              cpu: <h2>{currStats()!.cpu_perc.toFixed(1)}%</h2>
            </Grid>
            <Grid gap="0" placeItems="start center">
              mem:{" "}
              <h2>
                {(
                  (100 * currStats()!.mem_used_gb) /
                  currStats()!.mem_total_gb
                ).toFixed(1)}
                %
              </h2>
            </Grid>
            <Grid gap="0" placeItems="start center">
              disk:{" "}
              <h2>
                {(
                  (100 * currStats()!.disk.used_gb) /
                  currStats()!.disk.total_gb
                ).toFixed(1)}
                %
              </h2>
            </Grid>
          </Show>
        </Flex>
        <Selector
          selected={timelength()}
          items={TIMELENGTHS}
          onSelect={(selected) => setTimelength(selected as Timelength)}
        />
      </Flex>
      <Show
        when={stats()}
        fallback={
          <div style={{ "place-self": "center" }}>
            <Loading type="three-dot" />
          </div>
        }
      >
        <Grid class={s.Charts}>
          <CpuChart stats={stats} />
          <MemChart stats={stats} />
          <DiskChart stats={stats} />
        </Grid>
      </Show>
    </Grid>
  );
};

export default Stats;

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
      <Grid gap="0" class="card dark shadow" style={{ height: "fit-content" }}>
        <h2>cpu %</h2>
        <LightweightChart
          class={s.LightweightChart}
          style={{ height: "200px" }}
          lines={() => [{ color: "#184e9f", line: line()! }]}
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
      <Grid gap="0" class="card dark shadow" style={{ height: "fit-content" }}>
        <Flex alignItems="center" justifyContent="space-between">
          <h2>memory %</h2>
          <Selector
            selected={selected()}
            items={["%", "GB"]}
            onSelect={setSelected}
          />
        </Flex>
        <LightweightChart
          class={s.LightweightChart}
          style={{ height: "200px" }}
          lines={() => [{ color: "#184e9f", line: line()! }]}
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
      <Grid gap="0" class="card dark shadow" style={{ height: "fit-content" }}>
        <Flex alignItems="center" justifyContent="space-between">
          <h2>disk %</h2>
          <Selector
            selected={selected()}
            items={["%", "GB"]}
            onSelect={setSelected}
          />
        </Flex>
        <LightweightChart
          class={s.LightweightChart}
          style={{ height: "200px" }}
          lines={() => [{ color: "#184e9f", line: line()! }]}
        />
      </Grid>
    </Show>
  );
};
