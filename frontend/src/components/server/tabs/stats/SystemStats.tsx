import { Component, createSignal, Show } from "solid-js";
import { pushNotification } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { combineClasses } from "../../../../util/helpers";
import Icon from "../../../shared/Icon";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import Loading from "../../../shared/loading/Loading";
import s from "./stats.module.scss";

const SystemStats: Component<{}> = (p) => {
  const { servers, serverStats } = useAppState();
  const [refreshingStats, setRefreshingStats] = createSignal(false);
  // const sysStats = () => serverStats.get(selected.id(), servers.get(selected.id()));
  // const loadStats = async () => {
  //   if (selected.id() && servers.get(selected.id())?.status === "OK") {
  //     setRefreshingStats(true);
  //     await serverStats.load(selected.id());
  //     setRefreshingStats(false);
  //     pushNotification("good", "system stats refreshed");
  //   }
  // };
  return (
    <Show when={true}>
      {/* <Grid class={combineClasses(s.StatsContainer, themeClass())}>
        <Flex justifyContent="space-between">
          <h1>system stats</h1>
          <Button
            class="blue"
            style={{ "justify-self": "end" }}
            onClick={loadStats}
          >
            <Show when={!refreshingStats()} fallback={<Loading />}>
              <Icon type="refresh" />
            </Show>
          </Button>
        </Flex>
        <Flex alignItems="center">
          <h2>cpu: </h2>
          <div>{sysStats()!.cpu}%</div>
        </Flex>
        <Flex alignItems="center">
          <h2>mem: </h2>
          <div>{sysStats()!.mem.usedMemPercentage}%</div>
          <div>
            (using {sysStats()!.mem.usedMemMb} mb of{" "}
            {sysStats()!.mem.totalMemMb} mb)
          </div>
        </Flex>
        <Flex>
          <h2>disk: </h2>
          <div>{sysStats()!.disk.usedPercentage}%</div>
          <div>
            (using {sysStats()!.disk.usedGb} gb of {sysStats()!.disk.totalGb}{" "}
            gb)
          </div>
        </Flex>
      </Grid> */}
    </Show>
  );
};

export default SystemStats;
