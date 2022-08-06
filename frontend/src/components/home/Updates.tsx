import { Component, For, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Grid from "../util/layout/Grid";
import s from "./home.module.scss";
import Update from "../update/Update";
import { combineClasses } from "../../util/helpers";
import { useAppDimensions } from "../../state/DimensionProvider";
import Button from "../util/Button";
import { useTheme } from "../../state/ThemeProvider";

const Updates: Component<{}> = () => {
  const { updates } = useAppState();
  const { isMobile } = useAppDimensions();
	const { themeClass } = useTheme();
  return (
    <Show when={updates.loaded()}>
      <Grid
        class={combineClasses(s.Updates, "card shadow", themeClass())}
        style={isMobile() ? { width: "100%" } : undefined}
      >
        <h1>updates</h1>
        <Grid class={combineClasses(s.UpdatesContainer, "scroller")}>
          <For each={updates.collection()!}>
            {(update) => <Update update={update} showName />}
          </For>
          <Show when={!updates.noMore()}>
            <Button
              class="grey"
              style={{ width: "100%" }}
              onClick={updates.loadMore}
            >
              load more
            </Button>
          </Show>
        </Grid>
      </Grid>
    </Show>
  );
};

export default Updates;
