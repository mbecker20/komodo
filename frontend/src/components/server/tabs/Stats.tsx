import { CommandLogError } from "@monitor/types";
import { Component, createEffect, createSignal, Show } from "solid-js";
import { pushNotification } from "../../..";
import { useAppState } from "../../../state/StateProvider";
import { useTheme } from "../../../state/ThemeProvider";
import { combineClasses } from "../../../util/helpers";
import { getServerStats } from "../../../util/query";
import Button from "../../util/Button";
import Icon from "../../util/Icon";
import Grid from "../../util/layout/Grid";
import Loading from "../../util/loading/Loading";
import s from "./stats.module.scss";

const Stats: Component<{}> = (p) => {
  const { selected } = useAppState();
  const [log, setLog] = createSignal<CommandLogError>();
  const [refreshing, setRefreshing] = createSignal(false);
  const load = () => {
    if (selected.id()) {
      getServerStats(selected.id()).then(setLog);
    }
  };
  const { themeClass } = useTheme();
  createEffect(load);
  return (
    <Grid
      placeItems={log() ? "start center" : "center"}
      style={{ overflow: "scroll", height: "100%" }}
    >
      <Show when={log()} fallback={<Loading type="three-dot" scale={0.8} />}>
        <Grid class={combineClasses(s.StatsContainer, themeClass())}>
          <Button
            class="blue"
            style={{ "justify-self": "end" }}
            onClick={async () => {
              setRefreshing(true);
              const log = await getServerStats(selected.id());
              setLog(log);
              setRefreshing(false);
              pushNotification("good", "stats refreshed");
            }}
          >
            <Show when={!refreshing()} fallback={<Loading />}>
              <Icon type="refresh" />
            </Show>
          </Button>
          <pre class={s.Stats}>{log()!.log.stdout}</pre>
        </Grid>
      </Show>
    </Grid>
  );
};

export default Stats;
