import { Component, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import Grid from "../../util/layout/Grid";
import s from "../topbar.module.css";
import Update from "./Update";

const Updates: Component<{}> = (p) => {
  const { updates } = useAppState();
  return (
    <Show when={updates.loaded()}>
      <Grid class={s.Updates}>
        <div style={{ "font-size": "1.5rem", "font-weight": 500, "place-self": "center end" }}>updates</div>
        <For each={updates.collection()!}>
          {(update) => <Update update={update} />}
        </For>
      </Grid>
    </Show>
  );
};

export default Updates;
