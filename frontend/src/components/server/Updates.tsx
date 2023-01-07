import {
  Component,
  createEffect,
  For,
  onCleanup,
  Show,
} from "solid-js";
import { useUpdates } from "../../state/hooks";
import { useAppState } from "../../state/StateProvider";
import Update from "../update/Update";
import Grid from "../shared/layout/Grid";
import { combineClasses } from "../../util/helpers";
import { useParams } from "@solidjs/router";

const Updates: Component<{}> = (p) => {
  const { ws } = useAppState();
  const params = useParams();
  const updates = useUpdates({ type: "Server", id: params.id });
  let unsub = () => {};
  createEffect(() => {
    unsub();
    unsub = ws.subscribe([], (update) => {
      if (update.target.type === "Server" && update.target.id === params.id) {
        updates.addOrUpdate(update);
      }
    });
  });
  onCleanup(() => unsub());
  return (
    <Grid
      class={combineClasses("card shadow")}
      style={{ "min-width": "350px" }}
    >
      <h1>updates</h1>
      <Grid class="updates-container scroller">
        <For each={updates.collection()}>
          {(update) => <Update update={update} />}
        </For>
        <Show when={!updates.noMore()}>
          <button
            class="grey"
            style={{ width: "100%" }}
            onClick={() => updates.loadMore()}
          >
            load more
          </button>
        </Show>
      </Grid>
    </Grid>
  );
};

export default Updates;
