import { Component, createEffect, createSignal, Show, For } from "solid-js";
import { pushNotification } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { combineClasses } from "../../../../util/helpers";
import Icon from "../../../shared/Icon";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import Loading from "../../../shared/loading/Loading";
import s from "./stats.module.scss";

const DockerStats: Component<{}> = (p) => {
  // const [stats, setStats] = createSignal<DockerStat[]>();
  const [refreshing, setRefreshing] = createSignal(false);
  // const load = () => {
  //   if (selected.id()) {
  //     getServerStats(selected.id()).then(setStats);
  //   }
  // };
  // createEffect(load);
  // const { themeClass } = useTheme();
  return (
    <Show
      when={true}
      fallback={
        <Loading
          type="three-dot"
          scale={0.8}
          style={{ "place-self": "center" }}
        />
      }
    >
      {/* <Grid class={combineClasses(s.StatsContainer, themeClass())}>
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
        <Grid
          class="scroller"
          gap="0.5rem"
          style={{ padding: "0.5rem", "max-height": "30vh" }}
        >
          <For each={stats()}>
            {(stat) => (
              <Flex alignItems="center" justifyContent="space-between">
                <h2>{stat.Name}</h2>
                <Flex alignItems="center">
                  <div>cpu: {stat.CPUPerc}</div>
                  <div>mem: {stat.MemPerc}</div>
                </Flex>
              </Flex>
            )}
          </For>
        </Grid>
      </Grid> */}
    </Show>
  );
};

export default DockerStats;
