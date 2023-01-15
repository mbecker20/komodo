import { Component, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import Grid from "../../shared/layout/Grid";
import s from "../home.module.scss";
import Update from "./Update";
import { combineClasses } from "../../../util/helpers";
import { useAppDimensions } from "../../../state/DimensionProvider";

const Updates: Component<{}> = () => {
  const { updates } = useAppState();
  const { isMobile } = useAppDimensions();
  return (
    <Show when={updates.loaded()}>
      <Grid
        class={combineClasses(s.Updates, "card shadow")}
        style={{ width: "100%", "box-sizing": "border-box" }}
      >
        <h1>updates</h1>
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
      </Grid>
    </Show>
  );
};

export default Updates;
