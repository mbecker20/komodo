import { Server, StoredStats } from "@monitor/types";
import {
  Accessor,
  Component,
  createEffect,
  createSignal,
  Show,
} from "solid-js";
import { SolidApexCharts } from "solid-apexcharts";
import { useAppState } from "../../state/StateProvider";
import { useToggle } from "../../util/hooks";
import { getServerStatsHistory } from "../../util/query";
import Button from "../util/Button";
import Icon from "../util/Icon";
import Grid from "../util/layout/Grid";
import CenterMenu from "../util/menu/CenterMenu";
import { readableTimestamp } from "../../util/helpers";
import Flex from "../util/layout/Flex";
import Loading from "../util/loading/Loading";

const StatGraphs: Component<{ id: string }> = (p) => {
  const { servers } = useAppState();
  const [show, toggleShow] = useToggle();
  const name = () => servers.get(p.id)?.name;
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      target={<Icon type="timeline-line-chart" width="0.85rem" />}
      targetClass="blue"
      content={<Graphs id={p.id} />}
      title={`${name()} stats`}
    />
  );
};

const MOVEMENT = 50;

const Graphs: Component<{ id: string }> = (p) => {
  const { servers } = useAppState();
  const server = () => servers.get(p.id)!;
  const [stats, setStats] = createSignal<StoredStats[]>();
  const [offset, setOffset] = createSignal(0);
  const [reloadingLeft, setReloadingLeft] = createSignal(false);
  const [reloadingRight, setReloadingRight] = createSignal(false);
  const reloadStatsLeft = async () => {
    setReloadingLeft(true);
    const newOffset = offset() + MOVEMENT;
    const stats = await getServerStatsHistory(p.id, newOffset);
    setStats(stats.reverse());
    setOffset(newOffset);
    setReloadingLeft(false);
  };
  const reloadStatsRight = async () => {
    setReloadingRight(true);
    const newOffset = Math.max(offset() - MOVEMENT, 0);
    const stats = await getServerStatsHistory(p.id, newOffset);
    setStats(stats.reverse());
    setOffset(newOffset);
    setReloadingRight(false);
  };
  getServerStatsHistory(p.id).then((stats) => setStats(stats.reverse()));
  return (
    <Grid
      gap="0rem"
      placeItems="start center"
      style={{ "background-color": "white" }}
    >
      <Show when={stats()}>
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
        </Flex>
        <Graph stats={stats} field="cpu" server={server} />
        <Graph stats={stats} field="mem" server={server} />
        <Graph stats={stats} field="disk" server={server} />
      </Show>
    </Grid>
  );
};

const Graph: Component<{
  stats: Accessor<StoredStats[] | undefined>;
  field: "cpu" | "mem" | "disk";
  server: () => Server;
}> = (p) => {
  const options = () => ({
    chart: {
      id: "stats",
    },
    xaxis: {
      labels: {
        show: false,
      },
      categories: p.stats()!.map((stat) => readableTimestamp(stat.ts)),
    },
    // theme: {
    //   mode: isDark() ? "dark" : "light"
    // },
  });
  const series = () => [
    {
      name: p.field,
      data:
        p.field === "cpu"
          ? p.stats()!.map((stat) => stat.cpu)
          : p.field === "mem"
          ? p.stats()!.map((stat) => stat.mem.usedMemPercentage)
          : p.stats()!.map((stat) => stat.disk.usedPercentage),
    },
  ];

  return (
    <Grid placeItems="start center" gap="0rem">
      <h1 style={{ color: "black" }}>{p.field}</h1>
      <SolidApexCharts
        width="800"
        height="150"
        type="line"
        options={options()}
        series={series()}
      />
    </Grid>
  );
};

export default StatGraphs;
