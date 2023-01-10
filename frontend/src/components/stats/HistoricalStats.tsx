import { useParams } from "@solidjs/router";
import { Component, createEffect, createSignal, Match, Show, Switch } from "solid-js";
import { client } from "../..";
import { SystemStatsRecord, Timelength } from "../../types";
import { useLocalStorage } from "../../util/hooks";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Loading from "../shared/loading/Loading";
import Selector from "../shared/menu/Selector";
import { CpuChart, DiskChart, DiskIoCharts, MemChart, NetworkIoCharts, TempuratureChart } from "./Charts";
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
      <Flex alignItems="center" justifyContent="center">
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
              <NetworkIoCharts stats={stats} />
              <DiskIoCharts stats={stats} />
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

export default HistoricalStats;
