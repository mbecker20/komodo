import { Component, For, onCleanup, Show } from "solid-js";
import { useArray } from "../../state/hooks";
import { combineClasses } from "../../util/helpers";
import Grid from "../util/layout/Grid";
import s from "./deployment.module.css";
import Update from "../update/Update";
import { getUpdates } from "../../util/query";
import { useAppState } from "../../state/StateProvider";
import { ADD_UPDATE } from "../../state/actions";

const Updates: Component<{}> = (p) => {
  const { ws, selected } = useAppState();
  const selectedUpdates = useArray(() =>
    getUpdates({ deploymentID: selected.id() })
  );
  const unsub = ws.subscribe([ADD_UPDATE], ({ update }) => {
    if (update.deploymentID === selected.id()) {
      selectedUpdates.add(update);
    }
  });
  onCleanup(unsub);
  return (
    <Show
      when={
        selectedUpdates.loaded() &&
        (selectedUpdates.collection()?.length || 0) > 0
      }
    >
      <Grid class={combineClasses(s.Card, "shadow")}>
        <div class={s.ItemHeader}>updates</div>
        <Grid class={combineClasses(s.UpdatesContainer, "scroller")}>
          <For each={selectedUpdates.collection()}>
            {(update) => <Update update={update} showName={false} />}
          </For>
        </Grid>
      </Grid>
    </Show>
  );
};

export default Updates;
