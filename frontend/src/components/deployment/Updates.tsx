import { Component, createSignal, For, onCleanup, Show } from "solid-js";
import { useArray } from "../../state/hooks";
import Grid from "../util/layout/Grid";
import Update from "../update/Update";
import { getUpdates } from "../../util/query";
import { useAppState } from "../../state/StateProvider";
import { ADD_UPDATE, BUILD } from "@monitor/util";
import { useTheme } from "../../state/ThemeProvider";
import { combineClasses } from "../../util/helpers";
import Button from "../util/Button";

const Updates: Component<{}> = (p) => {
  const { ws, selected, deployments } = useAppState();
  const selectedUpdates = useArray(() =>
    getUpdates({ deploymentID: selected.id() })
  );
  const buildID = () => deployments.get(selected.id())?.buildID;
  onCleanup(
    ws.subscribe([ADD_UPDATE], ({ update }) => {
      if (
        update.deploymentID === selected.id() ||
        (buildID() && buildID() === update.buildID && update.operation === BUILD)
      ) {
        selectedUpdates.add(update);
      }
    })
  );
  const [noMoreUpdates, setNoMore] = createSignal(false);
  const loadMore = async () => {
    const offset = selectedUpdates.collection()?.length;
    if (offset) {
      const updates = await getUpdates({ offset, deploymentID: selected.id() });
      selectedUpdates.addManyToEnd(updates);
      if (updates.length !== 10) {
        setNoMore(true);
      }
    }
  };
  const { themeClass } = useTheme();
  return (
    <Show
      when={
        selectedUpdates.loaded() &&
        (selectedUpdates.collection()?.length || 0) > 0
      }
    >
      <Grid class={combineClasses("card shadow", themeClass())}>
        <h1>updates</h1>
        <Grid class="updates-container scroller">
          <For each={selectedUpdates.collection()}>
            {(update) => <Update update={update} showName={false} />}
          </For>
          <Show when={!noMoreUpdates()}>
            <Button class="grey" style={{ width: "100%" }} onClick={loadMore}>
              load more
            </Button>
          </Show>
        </Grid>
      </Grid>
    </Show>
  );
};

export default Updates;
