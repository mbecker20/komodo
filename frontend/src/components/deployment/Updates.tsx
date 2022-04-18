import { Component, createSignal, For, onCleanup, Show } from "solid-js";
import { useArray } from "../../state/hooks";
import Grid from "../util/layout/Grid";
import Update from "../update/Update";
import { getUpdates } from "../../util/query";
import { useAppState } from "../../state/StateProvider";
import { ADD_UPDATE } from "@monitor/util";

const Updates: Component<{}> = (p) => {
  const { ws, selected } = useAppState();
  const selectedUpdates = useArray(() =>
    getUpdates({ deploymentID: selected.id() })
  );
  const unsub = ws.subscribe([ADD_UPDATE], ({ update }) => {
    if (update.deploymentID === selected.id()) {
      selectedUpdates.add(update);
    }
  });
  onCleanup(unsub);
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
  return (
    <Show
      when={
        selectedUpdates.loaded() &&
        (selectedUpdates.collection()?.length || 0) > 0
      }
    >
      <Grid class="card shadow">
        <h1>updates</h1>
        <Grid class="updates-container scroller">
          <For each={selectedUpdates.collection()}>
            {(update) => <Update update={update} showName={false} />}
          </For>
          <Show when={!noMoreUpdates()}>
            <button class="grey" style={{ width: "100%" }} onClick={loadMore}>
              load more
            </button>
          </Show>
        </Grid>
      </Grid>
    </Show>
  );
};

export default Updates;
