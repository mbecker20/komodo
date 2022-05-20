import { SystemStats as SystemStatsType } from "@monitor/types";
import { Component, createEffect, createSignal, Show } from "solid-js";
import { pushNotification } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { getServerSystemStats } from "../../../../util/query";
import Button from "../../../util/Button";
import Icon from "../../../util/Icon";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Loading from "../../../util/loading/Loading";

const SystemStats: Component<{}> = (p) => {
  const { selected } = useAppState();
  const [sysStats, setSysStats] = createSignal<SystemStatsType>();
  const [refreshingStats, setRefreshingStats] = createSignal(false);
  const loadStats = () => {
    if (selected.id()) {
      getServerSystemStats(selected.id()).then(setSysStats);
    }
  };
  createEffect(loadStats);
  return (
    <Show when={sysStats()}>
      <Flex>
        <Grid style={{ height: "fit-content" }}>
          <h2>cpu: {sysStats()!.cpu}%</h2>
          <Flex alignItems="center">
            <h2>mem: {sysStats()!.mem.usedMemPercentage}%</h2>
            <h2>
              (using {sysStats()!.mem.usedMemMb} mb of{" "}
              {sysStats()!.mem.totalMemMb} mb)
            </h2>
          </Flex>
          <Flex>
            <h2>disk: {sysStats()!.disk.usedPercentage}%</h2>
            <h2>
              (using {sysStats()!.disk.usedGb} gb of {sysStats()!.disk.totalGb}{" "}
              gb)
            </h2>
          </Flex>
        </Grid>
        <Button
          class="blue"
          style={{ "justify-self": "end" }}
          onClick={async () => {
            setRefreshingStats(true);
            const stats = await getServerSystemStats(selected.id());
            setSysStats(stats);
            setRefreshingStats(false);
            pushNotification("good", "system stats refreshed");
          }}
        >
          <Show when={!refreshingStats()} fallback={<Loading />}>
            <Icon type="refresh" />
          </Show>
        </Button>
      </Flex>
    </Show>
  );
};

export default SystemStats;
