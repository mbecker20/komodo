import { useParams } from "@solidjs/router";
import { Accessor, Component, createSignal, Match, Setter, Show, Signal, Switch } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { Timelength } from "../../types";
import { useLocalStorage } from "../../util/hooks";
import Circle from "../shared/Circle";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import HoverMenu from "../shared/menu/HoverMenu";
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
  const [wsOpen, setWsOpen] = createSignal(false);
  return (
    <Grid class={s.Content}>
      <Grid class={s.HeaderArea} placeItems="center start">
        <Header view={view()} open={wsOpen()} />
        <Show when={view() === "historical"} fallback={<div />}>
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
        </Show>
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
          <CurrentStats setWsOpen={setWsOpen} />
        </Match>
        <Match when={view() === "historical"}>
          <HistoricalStats page={page} timelength={timelength} />
        </Match>
      </Switch>
    </Grid>
  );
};

export const Header: Component<{ view: string, open: boolean }> = (p) => {
  const { servers } = useAppState();
  const params = useParams();
  const server = () => servers.get(params.id);
  return (
    <Flex alignItems="center">
      <h1>{server()?.server.name} - system stats</h1>
      <Show when={p.view === "current"}>
        <HoverMenu
          target={
            <Circle
              size={1}
              class={p.open ? "green" : "red"}
              style={{ transition: "all 500ms ease-in-out" }}
            />
          }
          content={p.open ? "connected" : "disconnected"}
          position="right center"
        />
      </Show>
    </Flex>
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
