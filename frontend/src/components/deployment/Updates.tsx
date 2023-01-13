import { Component, createEffect, For, onCleanup, Show } from "solid-js";
import { useUpdates } from "../../state/hooks";
import Grid from "../shared/layout/Grid";
import Update from "../update/Update";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import { useParams } from "@solidjs/router";
import { Operation } from "../../types";
import Flex from "../shared/layout/Flex";
import Loading from "../shared/loading/Loading";

const Updates: Component<{}> = (p) => {
  const { ws, deployments } = useAppState();
  const params = useParams();
  const updates = useUpdates({ type: "Deployment", id: params.id });
  const buildID = () => deployments.get(params.id)?.deployment.build_id;
  let unsub = () => {};
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
    <Grid
      class={combineClasses("card shadow")}
      style={{ "min-width": "350px" }}
    >
      <h1>updates</h1>
      <Show
        when={updates.loaded()}
        fallback={
          <Flex justifyContent="center">
            <Loading type="three-dot" />
          </Flex>
        }
      >
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
      </Show>
    </Grid>
  );
};

export default Updates;
