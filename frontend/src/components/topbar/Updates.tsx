import { Component, For, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Grid from "../util/layout/Grid";
import s from "./topbar.module.scss";
import Update from "../update/Update";
import { combineClasses } from "../../util/helpers";

const Updates: Component<{}> = () => {
  const { updates } = useAppState();
  return (
    <Show when={updates.loaded()}>
      <Grid class={s.Updates}>
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
        </Grid>
      </Grid>
    </Show>
  );
};

export default Updates;
