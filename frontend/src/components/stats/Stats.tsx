import { A, useParams } from "@solidjs/router";
import {
  Component,
  Match,
  Show,
  Switch,
} from "solid-js";
import { MAX_PAGE_WIDTH } from "../..";
import { useAppState } from "../../state/StateProvider";
import { ServerStatus, Timelength } from "../../types";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Selector from "../shared/menu/Selector";
import CurrentStats from "./CurrentStats";
import HistoricalStats from "./HistoricalStats";
import { StatsProvider, useStatsState } from "./Provider";

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
    <Grid
      style={{
        width: "100%",
        "box-sizing": "border-box",
      }}
    >
      <Flex justifyContent="space-between" style={{ width: "100%" }}>
        <Header />
        <SysInfo />
      </Flex>
      <Show when={view() === "historical"}>
        <Flex alignItems="center" style={{ "place-self": "center" }}>
          <PageManager />
        </Flex>
      </Show>
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

export const Header: Component<{}> = (p) => {
  const { servers } = useAppState();
  const params = useParams();
  const server = () => servers.get(params.id);
  const { view, setView, timelength, setTimelength, setPage, pollRate, setPollRate } = useStatsState();
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
      <Grid gap="0" gridTemplateColumns="repeat(2, 1fr)">
        <button
          class={view() === "current" ? "selected" : "grey"}
          style={{ width: "100%" }}
          onClick={() => setView("current")}
        >
          current
        </button>
        <button
          class={view() === "historical" ? "selected" : "grey"}
          style={{ width: "100%" }}
          onClick={() => setView("historical")}
        >
          historical
        </button>
      </Grid>
      <Show when={view() === "historical"}>
        <Selector
          targetClass="grey"
          selected={timelength()}
          items={TIMELENGTHS}
          onSelect={(selected) => {
            setPage(0);
            setTimelength(selected as Timelength);
          }}
        />
      </Show>
      <Show when={view() === "current"}>
        <Flex gap="0.5rem" alignItems="center">
          <div>poll:</div>
          <Selector
            targetClass="grey"
            selected={pollRate()}
            items={[Timelength.OneSecond, Timelength.FiveSeconds]}
            onSelect={(selected) => {
              setPollRate(selected as Timelength);
            }}
          />
        </Flex>
      </Show>
    </Flex>
  );
};

const SysInfo = () => {
  const { sysInfo } = useStatsState();
  return (
    <Flex
      alignItems="center"
      style={{ "place-self": "center end", width: "fit-content" }}
    >
      <div>{sysInfo()?.os}</div>
      {/* <div>{sysInfo()?.kernel}</div> */}
      <div>{sysInfo()?.cpu_brand}</div>
      <div>{sysInfo()?.core_count} cores</div>
    </Flex>
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
