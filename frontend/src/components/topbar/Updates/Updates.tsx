import { Component, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import Grid from "../../shared/layout/Grid";
import Update from "./Update";

const Updates: Component<{}> = () => {
  const { updates } = useAppState();
  return (
    <Show when={updates.loaded()}>
      {/* <h1>updates</h1> */}
      <Grid>
        <For each={updates.collection()!}>
          {(update) => <Update update={update} />}
        </For>
        <Show when={!updates.noMore()}>
          <button
            class="grey"
            style={{ width: "100%" }}
            onClick={updates.loadMore}
          >
            load more
          </button>
        </Show>
      </Grid>
    </Show>
  );
};

export default Updates;
