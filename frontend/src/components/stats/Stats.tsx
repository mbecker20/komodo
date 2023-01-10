import { useParams } from "@solidjs/router";
import { Component, Match, Switch } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useLocalStorage } from "../../util/hooks";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Selector from "../shared/menu/Selector";
import CurrentStats from "./CurrentStats";
import HistoricalStats from "./HistoricalStats";
import s from "./stats.module.scss";

const VIEWS = [
  "current",
  "historical"
]

const Stats: Component<{}> = (p) => {
  const [view, setView] = useLocalStorage("current", "stats-view-v1");
  return (
    <Grid class={s.Content}>
      <Flex alignItems="center" justifyContent="space-between">
        <Header />
        <Selector
          targetClass="grey"
          selected={view()}
          items={VIEWS}
          onSelect={setView}
        />
      </Flex>
      <Switch>
        <Match when={view() === "current"}>
          <CurrentStats />
        </Match>
        <Match when={view() === "historical"}>
          <HistoricalStats />
        </Match>
      </Switch>
    </Grid>
  );
};

export const Header = () => {
  const { servers } = useAppState();
  const params = useParams();
  const server = () => servers.get(params.id);
  return (
    <Grid gap="0.1rem">
      <h1>{server()?.server.name} - system stats</h1>
    </Grid>
  );
}

export default Stats;
