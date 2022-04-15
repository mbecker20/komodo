import { Component, For, JSX, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Grid from "../util/layout/Grid";
import s from "./topbar.module.scss";
import Update from "../update/Update";
import { combineClasses, inPx } from "../../util/helpers";
import { useAppDimensions } from "../../state/DimensionProvider";

const Updates: Component<{}> = () => {
  const { updates } = useAppState();
  const { isMobile } = useAppDimensions();
  return (
    <Show when={updates.loaded()}>
      <Grid class={s.Updates} style={isMobile() ? { width: "100%" } : undefined}>
        <div
          style={{
            "font-size": "1.5rem",
            "font-weight": 500,
            "place-self": "center end",
          }}
        >
          updates
        </div>
        <Grid class={combineClasses(s.UpdatesContainer, "scroller")}>
          <For each={updates.collection()!}>
            {(update) => <Update update={update} showName />}
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
