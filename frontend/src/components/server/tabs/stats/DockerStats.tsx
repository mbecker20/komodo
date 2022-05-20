import { DockerStat } from "@monitor/types";
import { Component, createEffect, createSignal, Show, For } from "solid-js";
import { pushNotification } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { useTheme } from "../../../../state/ThemeProvider";
import { combineClasses } from "../../../../util/helpers";
import { getServerStats } from "../../../../util/query";
import Button from "../../../util/Button";
import Icon from "../../../util/Icon";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Loading from "../../../util/loading/Loading";
import s from "./stats.module.scss";

const DockerStats: Component<{}> = (p) => {
	const { selected } = useAppState();
	 const [stats, setStats] = createSignal<DockerStat[]>();
   const [refreshing, setRefreshing] = createSignal(false);
   const load = () => {
     if (selected.id()) {
       getServerStats(selected.id()).then(setStats);
     }
   };
   createEffect(load);
	const { themeClass } = useTheme();
	return (
    <Show when={stats()} fallback={<Loading type="three-dot" scale={0.8} style={{ "place-self": "center" }} />}>
      <Grid class={combineClasses(s.StatsContainer, themeClass())}>
        <Flex justifyContent="space-between">
          <h1>container stats</h1>
          <Button
            class="blue"
            onClick={async () => {
              setRefreshing(true);
              const stats = await getServerStats(selected.id());
              setStats(stats);
              setRefreshing(false);
              pushNotification("good", "stats refreshed");
            }}
          >
            <Show when={!refreshing()} fallback={<Loading />}>
              <Icon type="refresh" />
            </Show>
          </Button>
        </Flex>
        <Grid style={{ padding: "0.5rem" }}>
          <For each={stats()}>
            {(stat) => (
              <Flex alignItems="center" justifyContent="space-between">
                <h2>{stat.Name}</h2>
                <Flex alignItems="center">
                  <div>cpu: {stat.CPUPerc}</div>
                  <div>
                    mem: {stat.MemPerc}
                  </div>
                </Flex>
              </Flex>
            )}
          </For>
        </Grid>
      </Grid>
    </Show>
  );
}

export default DockerStats;