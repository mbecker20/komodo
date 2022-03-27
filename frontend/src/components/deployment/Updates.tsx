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
  const listener = ({ data }: { data: string }) => {
    const message = JSON.parse(data);
    if (
      message.type === ADD_UPDATE &&
      message.update.deploymentID === selected.id()
    ) {
      selectedUpdates.add(message.update);
    }
  };
  ws.socket.addEventListener("message", listener);
  onCleanup(() => {
    ws.socket.removeEventListener("message", listener);
  });
  return (
    <Show
      when={
        selectedUpdates.loaded() &&
        (selectedUpdates.collection()?.length || 0) > 0
      }
    >
      <Grid class={combineClasses(s.Updates, "shadow")}>
        <div class={s.ItemHeader}>updates</div>
        <Grid class={s.UpdatesContainer}>
          <For each={selectedUpdates.collection()}>
            {(update) => <Update update={update} showName={false} />}
          </For>
        </Grid>
      </Grid>
    </Show>
  );
};

export default Updates;
