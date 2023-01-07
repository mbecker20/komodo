import { useParams } from "@solidjs/router";
import { Component, createEffect, createSignal } from "solid-js";
import { client } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { Timelength } from "../../../../types";
import { useLocalStorage } from "../../../../util/hooks";
import Grid from "../../../shared/layout/Grid";

const TIMELENGTHS = [
  Timelength.OneMinute,
  Timelength.FiveMinutes,
  Timelength.FifteenMinutes,
  Timelength.OneHour,
  Timelength.SixHours,
  Timelength.OneDay,
];

const Stats: Component<{}> = (p) => {
  const { servers } = useAppState();
  const params = useParams();
  const [timelength, setTimelength] = useLocalStorage(
    Timelength.OneHour,
    "server-stats-timelength-v1"
  );
  const [stats, setStats] = createSignal();
  createEffect(() => {
    client.get_server_stats_history(params.id, { interval: timelength(), networks: true, components: true });
  });
  return <Grid style={{ width: "100%" }}></Grid>;
};

export default Stats;
