import { A, useParams } from "@solidjs/router";
import { Component, For, Match, Show, Switch } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { ServerStatus, Timelength } from "../../types";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Selector from "../shared/menu/Selector";
import CurrentStats from "./CurrentStats";
import HistoricalStats from "./HistoricalStats";
import { StatsProvider, useStatsState, StatsView } from "./Provider";

const TIMELENGTHS = [
  Timelength.FifteenSeconds,
  Timelength.OneMinute,
  Timelength.FiveMinutes,
  Timelength.FifteenMinutes,
  Timelength.OneHour,
  Timelength.SixHours,
  Timelength.TwelveHours,
  Timelength.OneDay,
];

const Stats = () => {
  return (
    <StatsProvider>
      <StatsComp />
    </StatsProvider>
  );
};

const StatsComp: Component<{}> = () => {
  const { view } = useStatsState();
  return (
    <Grid class="full-width">
      <Header />
      <Show when={view() === StatsView.Historical}>
        <Flex alignItems="center" style={{ "place-self": "center" }}>
          <PageManager />
        </Flex>
      </Show>
      <Switch>
        <Match when={view() === StatsView.Current}>
          <CurrentStats />
        </Match>
        <Match when={view() === StatsView.Historical}>
          <HistoricalStats />
        </Match>
        <Match when={view() === StatsView.Info}>
          <SysInfo />
        </Match>
      </Switch>
    </Grid>
  );
};

export const Header: Component<{}> = (p) => {
  const { servers } = useAppState();
  const params = useParams();
  const server = () => servers.get(params.id);
  const {
    view,
    setView,
    timelength,
    setTimelength,
    setPage,
    pollRate,
    setPollRate,
  } = useStatsState();
  return (
    <Flex alignItems="center" style={{ height: "fit-content" }}>
      <h1>{server()?.server.name}</h1>
      <A
        href={`/server/${params.id}`}
        class={
          server()?.server.enabled
            ? server()?.status === ServerStatus.Ok
              ? "green"
              : "red"
            : "blue"
        }
        style={{
          "border-radius": ".35rem",
          transition: "background-color 125ms ease-in-out",
        }}
        onClick={(e) => {
          e.stopPropagation();
        }}
      >
        {server()?.status.replaceAll("_", " ").toUpperCase()}
      </A>
      <Selector
        targetClass="blue"
        selected={view()}
        items={Object.values(StatsView)}
        onSelect={(v) => setView(v as StatsView)}
        position="bottom right"
      />
      <Show when={view() === "historical"}>
        <Selector
          targetClass="grey"
          selected={timelength()}
          items={TIMELENGTHS}
          itemMap={(t) => t.replaceAll("-", " ")}
          itemClass="full-width"
          onSelect={(selected) => {
            setPage(0);
            setTimelength(selected as Timelength);
          }}
          position="bottom right"
        />
      </Show>
      <Show when={view() === "current"}>
        <Selector
          targetClass="grey"
          label="poll: "
          selected={pollRate()}
          items={[Timelength.OneSecond, Timelength.FiveSeconds]}
          onSelect={(selected) => {
            setPollRate(selected as Timelength);
          }}
          position="bottom right"
        />
      </Show>
    </Flex>
  );
};

const SysInfo = () => {
  const { sysInfo } = useStatsState();
  const cards = () => {
    return [
      {
        label: "os",
        info: sysInfo()?.os,
      },
      {
        label: "cpu",
        info: sysInfo()?.cpu_brand,
      },
      {
        label: "core count",
        info: `${sysInfo()?.core_count} cores`,
      },
    ];
  };
  return (
    <Grid class="full-width" placeItems="center">
      <Grid class="card full-width" style={{ "max-width": "700px" }}>
        <For each={cards()}>
          {(c) => (
            <Flex class="full-width" justifyContent="space-between">
              <div>{c.label}</div>
              <div>{c.info}</div>
            </Flex>
          )}
        </For>
      </Grid>
    </Grid>
  );
};

const PageManager: Component<{}> = (p) => {
  const { page, setPage } = useStatsState();
  return (
    <Flex
      class="card light shadow"
      alignItems="center"
      style={{ padding: "0.5rem" }}
    >
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
  );
};

export default Stats;
