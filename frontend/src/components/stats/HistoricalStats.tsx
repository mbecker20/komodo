import { useParams } from "@solidjs/router";
import {
  Component,
  createEffect,
  createSignal,
  Show,
} from "solid-js";
import { client } from "../..";
import { SystemStatsRecord } from "../../types";
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
  TempuratureChart,
} from "./Charts";
import { useStatsState } from "./Provider";
import s from "./stats.module.scss";

const HistoricalStats: Component<{
}> = (p) => {
  const params = useParams();
  const { timelength, page } = useStatsState();
  const [stats, setStats] = createSignal<SystemStatsRecord[]>();
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
    <Grid class={s.Content} placeItems="start center">
      <Show when={stats()} fallback={<Loading type="three-dot" />}>
        <SimpleTabs
          localStorageKey="historical-stats-view-v3"
          defaultSelected="basic"
          containerStyle={{ width: "100%" }}
          tabs={[
            {
              title: "io",
              element: () => (
                <Grid class={s.Charts}>
                  <NetworkRecvChart stats={stats} />
                  <NetworkSentChart stats={stats} />
                  <DiskReadChart stats={stats} />
                  <DiskWriteChart stats={stats} />
                </Grid>
              ),
            },
            {
              title: "basic",
              element: () => (
                <Grid class={s.Charts}>
                  <CpuChart stats={stats} />
                  <MemChart stats={stats} />
                  <LoadChart stats={stats} />
                  <DiskChart stats={stats} />
                  {/* <CpuFreqChart stats={stats} /> */}
                </Grid>
              ),
            },
            {
              title: "temp",
              element: () => (
                <Grid class={s.Charts}>
                  <TempuratureChart stats={stats} />
                </Grid>
              ),
            },
          ]}
        />
      </Show>
    </Grid>
  );
};

export default HistoricalStats;
