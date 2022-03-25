import { Component, For, Show } from "solid-js";
import { useUpdates } from "../../../state/hooks";
import { useAppState } from "../../../state/StateProvider";
import { combineClasses } from "../../../util/helpers";
import Grid from "../../util/layout/Grid";
import s from "../deployment.module.css";
import Update from "./Update";

const Updates: Component<{ deploymentID: string }> = (p) => {
  const { updates, ws } = useAppState();
  const selectedUpdates = useUpdates({ deploymentID: p.deploymentID });

  return (
    <Show
      when={selectedUpdates.loaded() && selectedUpdates.collection.length > 0}
    >
      <Grid class={combineClasses(s.Updates, "shadow")}>
        <div class={s.ItemHeader}>updates</div>
        <For each={selectedUpdates.collection()}>
          {(update) => <Update update={update} />}
        </For>
      </Grid>
    </Show>
  );
};

export default Updates;
