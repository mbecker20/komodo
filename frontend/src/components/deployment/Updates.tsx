import { Component, createEffect, For, onCleanup, Show } from "solid-js";
import { useUpdates } from "../../state/hooks";
import Grid from "../shared/layout/Grid";
import Update from "../update/Update";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import { useParams } from "@solidjs/router";
import { Operation } from "../../types";

const Updates: Component<{}> = (p) => {
  const { ws, deployments } = useAppState();
  const params = useParams();
  const updates = useUpdates({ type: "Deployment", id: params.id });
  const buildID = () => deployments.get(params.id)?.deployment.build_id;
  let unsub = () => {}
  createEffect(() => {
    unsub();
    unsub = ws.subscribe([], (update) => {
      if (
        update.target.id === params.id ||
        (buildID() &&
          buildID() === update.target.id &&
          update.operation === Operation.BuildBuild)
      ) {
        updates.addOrUpdate(update);
      }
    });
  });
  onCleanup(() => unsub());
  return (
    <Show
      when={
        updates.loaded() &&
        (updates.collection()?.length || 0) > 0
      }
    >
      <Grid class={combineClasses("card shadow")}>
        <h1>updates</h1>
        <Grid class="updates-container scroller">
          <For each={updates.collection()}>
            {(update) => <Update update={update} />}
          </For>
          <Show when={!updates.noMore()}>
            <button class="grey" style={{ width: "100%" }} onClick={() => updates.loadMore()}>
              load more
            </button>
          </Show>
        </Grid>
      </Grid>
    </Show>
  );
};

export default Updates;
