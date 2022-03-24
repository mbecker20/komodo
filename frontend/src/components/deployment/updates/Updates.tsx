import { Component, For, Show } from "solid-js";
import { useUpdates } from "../../../state/hooks";
import { useAppState } from "../../../state/StateProvider";
import Grid from "../../util/layout/Grid";
import s from "../deployment.module.css";
import Update from "./Update";

const Updates: Component<{ deploymentID: string }> = (p) => {
  const { updates, ws } = useAppState();
  const selectedUpdates = useUpdates({ deploymentID: p.deploymentID });

  return (
    <Grid class={s.Updates}>
      <div class={s.ItemHeader}>updates</div>
      <Show when={selectedUpdates.loaded()}>
        <For each={selectedUpdates.collection()}>
          {(update) => <Update update={update} />}
        </For>
      </Show>
    </Grid>
  );
};

export default Updates;
