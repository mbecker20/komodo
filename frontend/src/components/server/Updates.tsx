import { Component, createSignal, For, onCleanup, Show } from "solid-js";
import { useArray } from "../../state/hooks";
import { useAppState } from "../../state/StateProvider";
import Update from "../update/Update";
import Grid from "../shared/layout/Grid";
import { combineClasses } from "../../util/helpers";
import { useParams } from "@solidjs/router";
import { client } from "../..";

const Updates: Component<{}> = (p) => {
	const { ws } = useAppState();
  const { id } = useParams();
  const selectedUpdates = useArray(() =>
    client.list_updates(0, { type: "Server", id })
  );
  const unsub = ws.subscribe([], (update) => {
    if (update.target.type === "Server" && update.target.id === id) {
      selectedUpdates.add(update);
    }
  });
  onCleanup(unsub);
  const [noMoreUpdates, setNoMore] = createSignal(false);
  const loadMore = async () => {
    const offset = selectedUpdates.collection()?.length;
    if (offset) {
      const updates = await client.list_updates(offset, { type: "Server", id });
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
      <Grid class={combineClasses("card shadow")}>
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
}

export default Updates;