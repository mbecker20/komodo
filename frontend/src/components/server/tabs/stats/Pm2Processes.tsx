import { PM2Process } from "@monitor/types";
import { Component, createEffect, createSignal, For, Show } from "solid-js";
import { pushNotification } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { getServerPm2Processes } from "../../../../util/query";
import Button from "../../../util/Button";
import Icon from "../../../util/Icon";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Loading from "../../../util/loading/Loading";

const Pm2Processes: Component<{}> = (p) => {
  const { selected } = useAppState();
  const [pm2Proc, setPm2Proc] = createSignal<PM2Process[]>();
  const [refreshing, setRefreshing] = createSignal(false);
  const loadPm2 = () => {
    if (selected.id()) {
      getServerPm2Processes(selected.id()).then(setPm2Proc);
    }
  };
  createEffect(loadPm2);
  return (
    <Show when={pm2Proc() && pm2Proc()!.length > 0}>
      <Grid>
        <Flex>
          <h2>pm2 processes</h2>
          <Button
            class="blue"
            onClick={async () => {
              setRefreshing(true);
              const processes = await getServerPm2Processes(selected.id());
              setPm2Proc(processes);
              setRefreshing(false);
              pushNotification("good", "processes refreshed");
            }}
          >
            <Show when={!refreshing()} fallback={<Loading />}>
              <Icon type="refresh" />
            </Show>
          </Button>
        </Flex>
        <For each={pm2Proc()}>
          {(process) => (
            <Flex justifyContent="space-between" alignItems="center">
              <div>{process.name}</div>
              <Flex alignItems="center">
                <div>status: {process.status}</div>
                <div>cpu: {process.cpu}%</div>
                <div>
                  mem: {process.memory ? process.memory / 1024000 : "unknown"}
                </div>
              </Flex>
            </Flex>
          )}
        </For>
      </Grid>
    </Show>
  );
};

export default Pm2Processes;
