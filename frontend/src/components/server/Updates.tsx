import { Component, createSignal, For, onCleanup, Show } from "solid-js";
import { ADD_UPDATE } from "@monitor/util";
import { useArray } from "../../state/hooks";
import { useAppState } from "../../state/StateProvider";
import { getUpdates } from "../../util/query";
import Update from "../update/Update";
import Grid from "../util/layout/Grid";
import { useTheme } from "../../state/ThemeProvider";
import { combineClasses } from "../../util/helpers";
import Button from "../util/Button";

const Updates: Component<{}> = (p) => {
	const { ws, selected } = useAppState();
  const selectedUpdates = useArray(() =>
    getUpdates({ serverID: selected.id() })
  );
  const unsub = ws.subscribe([ADD_UPDATE], ({ update }) => {
    if (update.serverID === selected.id()) {
      selectedUpdates.add(update);
    }
  });
  onCleanup(unsub);
  const [noMoreUpdates, setNoMore] = createSignal(false);
  const loadMore = async () => {
    const offset = selectedUpdates.collection()?.length;
    if (offset) {
      const updates = await getUpdates({ offset, serverID: selected.id() });
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
}

export default Updates;