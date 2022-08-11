import { StoredStats } from "@monitor/types";
import { Accessor, Component, createSignal, Show } from "solid-js";
import { SolidApexCharts } from "solid-apexcharts";
import { useAppState } from "../../state/StateProvider";
import { useToggle } from "../../util/hooks";
import { getServerStatsHistory } from "../../util/query";
import Button from "../util/Button";
import Icon from "../util/Icon";
import Grid from "../util/layout/Grid";
import CenterMenu from "../util/menu/CenterMenu";
import { readableTimestamp } from "../../util/helpers";

const StatGraphs: Component<{ id: string }> = (p) => {
  const [show, toggleShow] = useToggle();
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      target={<Icon type="timeline-line-chart" width="0.85rem" />}
      targetClass="blue"
      content={<Graphs id={p.id} />}
    />
  );
};

const Graphs: Component<{ id: string }> = (p) => {
  const { servers } = useAppState();
  const server = () => servers.get(p.id)!;
  const [stats, setStats] = createSignal<StoredStats[]>();
  const [reloading, setReloading] = createSignal(false);
  const reloadStats = async () => {
    setReloading(true);
    const stats = await getServerStatsHistory(p.id);
    setStats(stats);
    setReloading(false);
  };
  getServerStatsHistory(p.id).then(setStats);
  return (
    <Grid placeItems="center start">
      <h1>{server().name}</h1>
      <Show when={stats()}>
        <Graph stats={stats} field="cpu" />
      </Show>
    </Grid>
  );
};

const Graph: Component<{
  stats: Accessor<StoredStats[] | undefined>;
  field: string;
}> = (p) => {
  const options = () => ({
    chart: {
      id: "mychart",
    },
    xaxis: p.stats()!.map((stat) => readableTimestamp(stat.ts / 1000))
  });
  const series = () => [
    {
      name: "CPU Usage",
      data: p.stats()!.map((stat) => stat.cpu),
    },
  ];
    
  return (
    <SolidApexCharts
      width="500"
      type="line"
      options={options()}
      series={series() || []}
    />
  );
};

export default StatGraphs;
