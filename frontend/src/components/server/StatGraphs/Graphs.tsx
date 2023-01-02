import { Accessor, Component, createSignal, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import Icon from "../../shared/Icon";
import Grid from "../../shared/layout/Grid";
import { readableTimestamp } from "../../../util/helpers";
import Flex from "../../shared/layout/Flex";
import Loading from "../../shared/loading/Loading";
import { ApexOptions } from "apexcharts";
import ApexChart from "../../shared/ApexChart";

const MOVEMENT = 500;
const NUM_PTS = 1000;
const SKIP = 1;

const Graphs: Component<{ id: string }> = (p) => {
  // const { servers } = useAppState();
  // const server = () => servers.get(p.id)!;
  // const [stats, setStats] = createSignal<StoredStats[]>();
  // const [offset, setOffset] = createSignal(0);
  // const [reloadingLeft, setReloadingLeft] = createSignal(false);
  // const [reloadingRight, setReloadingRight] = createSignal(false);
  // const [reloadingReset, setReloadingReset] = createSignal(false);
  // const reloadStatsLeft = async () => {
  //   setReloadingLeft(true);
  //   const newOffset = offset() + MOVEMENT;
  //   const stats = await getServerStatsHistory(p.id, newOffset, NUM_PTS, SKIP);
  //   setStats(stats.reverse());
  //   setOffset(newOffset);
  //   setReloadingLeft(false);
  // };
  // const reloadStatsRight = async () => {
  //   setReloadingRight(true);
  //   const newOffset = Math.max(offset() - MOVEMENT, 0);
  //   const stats = await getServerStatsHistory(p.id, newOffset, NUM_PTS, SKIP);
  //   setStats(stats.reverse());
  //   setOffset(newOffset);
  //   setReloadingRight(false);
  // };
  // const reloadStatsReset = async () => {
  //   setReloadingReset(true);
  //   const stats = await getServerStatsHistory(p.id, 0, NUM_PTS, SKIP);
  //   setStats(stats.reverse());
  //   setOffset(0);
  //   setReloadingReset(false);
  // };
  // getServerStatsHistory(p.id, 0, NUM_PTS, SKIP).then((stats) =>
  //   setStats(stats.reverse())
  // );
  return (
    <Grid
      gap="0rem"
      placeItems="start center"
      style={{ "background-color": "white" }}
    >
      {/* <Show when={stats()}>
        <Flex
          justifyContent="space-between"
          style={{ margin: "1rem", width: "60%" }}
        >
          <Show
            when={!reloadingLeft()}
            fallback={
              <Button class="grey">
                <Loading type="three-dot" scale={0.2} />
              </Button>
            }
          >
            <Button
              class="grey"
              onClick={(e) => {
                e.stopPropagation();
                reloadStatsLeft();
              }}
            >
              <Icon type="arrow-left" />
            </Button>
          </Show>
          <Flex alignItems="center" style={{ width: "fit-content" }}>
            <Show
              when={!reloadingRight()}
              fallback={
                <Button class="grey">
                  <Loading type="three-dot" scale={0.2} />
                </Button>
              }
            >
              <Button
                class="grey"
                onClick={(e) => {
                  e.stopPropagation();
                  reloadStatsRight();
                }}
              >
                <Icon type="arrow-right" />
              </Button>
            </Show>
            <Show
              when={!reloadingReset()}
              fallback={
                <Button class="grey">
                  <Loading type="three-dot" scale={0.2} />
                </Button>
              }
            >
              <Button
                class="grey"
                onClick={(e) => {
                  e.stopPropagation();
                  reloadStatsReset();
                }}
              >
                <Icon type="double-chevron-right" />
              </Button>
            </Show>
          </Flex>
        </Flex>
        <Graph stats={stats} field="cpu" server={server} />
        <Graph stats={stats} field="mem" server={server} />
        <Graph stats={stats} field="disk" server={server} />
      </Show> */}
    </Grid>
  );
};

const Graph: Component<{
  // stats: Accessor<StoredStats[] | undefined>;
  // field: "cpu" | "mem" | "disk";
  // server: () => Server;
}> = (p) => {
  // const options: () => ApexOptions = () => ({
  //   chart: {
  //     id: "stats",
  //     type: "line",
  //     width: 800,
  //     height: 150,
  //     events: {},
  //   },
  //   xaxis: {
  //     labels: {
  //       show: false,
  //     },
  //     categories: p.stats()!.map((stat) => readableTimestamp(stat.ts)),
  //   },
  //   series: [
  //     {
  //       name: p.field,
  //       data:
  //         p.field === "cpu"
  //           ? p.stats()!.map((stat) => stat.cpu)
  //           : p.field === "mem"
  //           ? p.stats()!.map((stat) => stat.mem.usedMemPercentage)
  //           : p.stats()!.map((stat) => stat.disk.usedPercentage),
  //     },
  //   ],
  //   // theme: {
  //   //   mode: isDark() ? "dark" : "light"
  //   // },
  // });

  return (
    <Grid placeItems="start center" gap="0rem">
      {/* <h1 style={{ color: "black" }}>{p.field}</h1>
      <ApexChart options={options()} /> */}
    </Grid>
  );
};

export default Graphs;
