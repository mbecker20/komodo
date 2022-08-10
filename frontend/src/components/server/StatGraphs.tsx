import { StoredStats } from "@monitor/types";
import { Component, createSignal } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useToggle } from "../../util/hooks";
import { getServerStatsHistory } from "../../util/query";
import Button from "../util/Button";
import Icon from "../util/Icon";
import Grid from "../util/layout/Grid";
import CenterMenu from "../util/menu/CenterMenu";

const StatGraphs: Component<{ id: string }> = (p) => {
  const [show, toggleShow] = useToggle();
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      target={
        <Button class="blue" onClick={toggleShow}>
          <Icon type="timeline-line-chart" />
        </Button>
      }
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
    </Grid>
  );
};

export default StatGraphs;
