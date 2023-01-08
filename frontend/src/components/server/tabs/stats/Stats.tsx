import { useParams } from "@solidjs/router";
import {
  Accessor,
  Component,
  createEffect,
  createSignal,
  Show,
} from "solid-js";
import { client } from "../../../..";
import { SystemStats, SystemStatsRecord, Timelength } from "../../../../types";
import {
  convertTsMsToLocalUnixTsInSecs,
  get_to_one_sec_divisor,
} from "../../../../util/helpers";
import { useLocalStorage } from "../../../../util/hooks";
import Icon from "../../../shared/Icon";
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
  Timelength.TwelveHours,
  Timelength.OneDay,
];

const Stats: Component<{}> = (p) => {
  const params = useParams();
  const [timelength, setTimelength] = useLocalStorage(
    Timelength.OneMinute,
    "server-stats-timelength-v3"
  );
  const [currStats, setCurrStats] = createSignal<SystemStats>();
  const [loadingCurr, setLoadingCurr] = createSignal(false);
  const [stats, setStats] = createSignal<SystemStatsRecord[]>();
  const [page, setPage] = createSignal(0);
  const load_curr_stats = () => {
    setLoadingCurr(true);
    client.get_server_stats(params.id).then((stats) => {
      setCurrStats(stats);
      setLoadingCurr(false);
    });
  };
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
  createEffect(() => {
    load_curr_stats();
  })
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
        // justifyContent="space-between"
      >
        <Flex class="card light shadow" alignItems="center">
          <Show when={currStats()} fallback={<Loading type="three-dot" />}>
            <Grid gap="0" placeItems="start center">
              cpu: <h2>{currStats()!.cpu_perc.toFixed(1)}%</h2>
            </Grid>
            <Grid gap="0" placeItems="start center">
              mem:
              <div>{currStats()!.mem_total_gb.toFixed(1)} GB</div>
              <h2>
                {(
                  (100 * currStats()!.mem_used_gb) /
                  currStats()!.mem_total_gb
                ).toFixed(1)}
                % full
              </h2>
            </Grid>
            <Grid gap="0" placeItems="start center">
              disk:
              <div>{currStats()!.disk.total_gb.toFixed(1)} GB</div>
              <h2>
                {(
                  (100 * currStats()!.disk.used_gb) /
                  currStats()!.disk.total_gb
                ).toFixed(1)}
                % full
              </h2>
            </Grid>
            <button class="blue" onClick={load_curr_stats}>
              <Show when={!loadingCurr()} fallback={<Loading />}>
                <Icon type="refresh" />
              </Show>
            </button>
          </Show>
        </Flex>
        <Flex class="card light shadow" alignItems="center">
          <button class="darkgrey" onClick={() => {
            setPage(page => page + 1);
          }}>
            <Icon type="chevron-left" />
          </button>
          <button class="darkgrey" onClick={() => {
            setPage(page => page > 0 ? page - 1 : 0);
          }}>
            <Icon type="chevron-right" />
          </button>
          <button class="darkgrey" onClick={() => {
            setPage(0)
          }}>
            <Icon type="double-chevron-right" />
          </button>
          <div>page: {page() + 1}</div>
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
          <NetworkChart stats={stats} />
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
        <h2>cpu</h2>
        <LightweightChart
          class={s.LightweightChart}
          style={{ height: "200px" }}
          lines={() => [{ title: "%", color: "#184e9f", line: line()! }]}
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
          <h2>memory</h2>
          <Selector
            selected={selected()}
            items={["%", "GB"]}
            onSelect={setSelected}
          />
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
          <h2>disk</h2>
          <Selector
            selected={selected()}
            items={["%", "GB"]}
            onSelect={setSelected}
          />
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

const NetworkChart: Component<{
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
      <Grid gap="0" class="card dark shadow" style={{ height: "fit-content" }}>
        <Flex alignItems="center" justifyContent="space-between">
          <h2>network kb/s</h2>
        </Flex>
        <LightweightChart
          class={s.LightweightChart}
          style={{ height: "200px" }}
          lines={() => [
            { title: "recv", color: "#41764c", line: recv_line()! },
            { title: "send", color: "#952E23", line: trans_line()! },
          ]}
        />
      </Grid>
    </Show>
  );
};
