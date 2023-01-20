import { Component, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import Flex from "../../shared/layout/Flex";
import Grid from "../../shared/layout/Grid";
import Loading from "../../shared/loading/Loading";
import Update from "./Update";

const Updates: Component<{}> = () => {
  const { updates } = useAppState();
  return (
    <Grid class="card shadow" style={{ "flex-grow": 1 }}>
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
    </Grid>
  );
};

export default Updates;
