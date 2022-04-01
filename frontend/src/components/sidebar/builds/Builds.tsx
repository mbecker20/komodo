import { Component, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import { combineClasses } from "../../../util/helpers";
import Grid from "../../util/layout/Grid";
import s from "../sidebar.module.css";
import Build from "./Build";
import NewBuild from "./NewBuild";

const Builds: Component<{}> = (p) => {
  const { builds } = useAppState();
  const { permissions } = useUser();
  return (
    <Grid gap=".5rem" class={combineClasses(s.Deployments)}>
      <For each={builds.ids()}>{(id) => <Build id={id} />}</For>
      <Show when={permissions() >= 1}>
        <NewBuild />
      </Show>
    </Grid>
  );
};

export default Builds;
