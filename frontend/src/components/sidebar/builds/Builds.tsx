import { Component, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { combineClasses } from "../../../util/helpers";
import { useLocalStorageToggle } from "../../../util/hooks";
import CreateBuild from "../../create/Build";
import Icon from "../../util/icons/Icon";
import Flex from "../../util/layout/Flex";
import Grid from "../../util/layout/Grid";
import s from "../sidebar.module.css";
import Build from "./Build";

const Builds: Component<{}> = (p) => {
  const { builds } = useAppState();
  const [open, toggleOpen] = useLocalStorageToggle("builds");
  return (
    <div class={combineClasses(s.Server, "shadow")}>
      <button
        class={combineClasses(s.ServerButton, "shadow")}
        onClick={toggleOpen}
      >
        <Flex>
          <Icon type="chevron-down" width="1rem" />
          <div>builds</div>
        </Flex>
      </button>
      <Show when={open()}>
        <Grid
          gap=".5rem"
          class={combineClasses(s.Deployments, open() ? s.Enter : s.Exit)}
        >
          <For each={builds.ids()}>{(id) => <Build id={id} />}</For>
          <CreateBuild />
        </Grid>
      </Show>
    </div>
  );
};

export default Builds;
