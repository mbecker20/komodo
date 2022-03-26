import { Component, For, Show } from "solid-js";
import { useArray, useUpdates } from "../../state/hooks";
import { combineClasses } from "../../util/helpers";
import Grid from "../util/layout/Grid";
import s from "./deployment.module.css";
import Update from "../update/Update";
import { getUpdates } from "../../util/query";

const Updates: Component<{ deploymentID: string }> = (p) => {
  const selectedUpdates = useArray(() =>
    getUpdates({ deploymentID: p.deploymentID })
  );
  return (
    <Show
      when={
        selectedUpdates.loaded() &&
        (selectedUpdates.collection()?.length || 0) > 0
      }
    >
      <Grid class={combineClasses(s.Updates, "shadow")}>
        <div class={s.ItemHeader}>updates</div>
        <For each={selectedUpdates.collection()}>
          {(update) => <Update update={update} showName={false} />}
        </For>
      </Grid>
    </Show>
  );
};

export default Updates;
