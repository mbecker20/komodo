import { useParams } from "@solidjs/router";
import { Accessor, Component, createSignal, Match, Setter, Signal, Switch } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { Timelength } from "../../types";
import { useLocalStorage } from "../../util/hooks";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Selector from "../shared/menu/Selector";
import CurrentStats from "./CurrentStats";
import HistoricalStats from "./HistoricalStats";
import s from "./stats.module.scss";

const VIEWS = [
  "current",
  "historical"
];

const TIMELENGTHS = [
  Timelength.OneMinute,
  Timelength.FiveMinutes,
  Timelength.FifteenMinutes,
  Timelength.OneHour,
  Timelength.SixHours,
  Timelength.TwelveHours,
  Timelength.OneDay,
];

const Stats: Component<{}> = () => {
  const [view, setView] = useLocalStorage("current", "stats-view-v1");
  const [timelength, setTimelength] = useLocalStorage(
    Timelength.OneMinute,
    "stats-timelength-v3"
  );
  const [page, setPage] = createSignal(0);
  return (
    <Grid class={s.Content}>
      <Grid class={s.HeaderArea} placeItems="center start">
        <Header />
        <Flex alignItems="center" style={{ "place-self": "center" }}>
          <PageManager page={page} setPage={setPage} />
          <Selector
            targetClass="grey"
            selected={timelength()}
            items={TIMELENGTHS}
            onSelect={(selected) => {
              setPage(0);
              setTimelength(selected as Timelength);
            }}
          />
        </Flex>
        <Selector
          containerStyle={{ "place-self": "center end" }}
          targetClass="grey"
          selected={view()}
          items={VIEWS}
          onSelect={setView}
          position="bottom right"
        />
      </Grid>
      <Switch>
        <Match when={view() === "current"}>
          <CurrentStats />
        </Match>
        <Match when={view() === "historical"}>
          <HistoricalStats page={page} timelength={timelength} />
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

const PageManager: Component<{ page: Accessor<number>, setPage: Setter<number> }> = (p) => {
  return (
    <Flex class="card light shadow" alignItems="center" style={{ padding: "0.5rem" }}>
      <button
        class="darkgrey"
        onClick={() => {
          p.setPage((page) => page + 1);
        }}
      >
        <Icon type="chevron-left" />
      </button>
      <button
        class="darkgrey"
        onClick={() => {
          p.setPage((page) => (page > 0 ? page - 1 : 0));
        }}
      >
        <Icon type="chevron-right" />
      </button>
      <button
        class="darkgrey"
        onClick={() => {
          p.setPage(0);
        }}
      >
        <Icon type="double-chevron-right" />
      </button>
      <div>page: {p.page() + 1}</div>
    </Flex>
  );
}

export default Stats;
