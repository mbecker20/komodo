import { Component, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useTheme } from "../../../state/ThemeProvider";
import { useUser } from "../../../state/UserProvider";
import { combineClasses } from "../../../util/helpers";
import Flex from "../../util/layout/Flex";
import Grid from "../../util/layout/Grid";
import s from "../sidebar.module.scss";
import Build from "./Build";
import NewBuild from "./NewBuild";

const Builds: Component<{}> = (p) => {
  const { builds } = useAppState();
  const { permissions, username } = useUser();
  const { themeClass } = useTheme();
  const filteredBuildIds = () =>
    builds
      .ids()
      ?.filter(
        (id) =>
          permissions() > 1 || builds.get(id)!.owners.includes(username()!)
      );
  return (
    <Grid gap=".5rem" class={combineClasses(s.Deployments, "shadow", themeClass())}>
      <Show
        when={filteredBuildIds() && (filteredBuildIds() as string[]).length === 0}
      >
        <Flex justifyContent="center">no builds</Flex>
      </Show>
      <For each={filteredBuildIds()}>{(id) => <Build id={id} />}</For>
      <Show when={permissions() >= 1}>
        <NewBuild />
      </Show>
    </Grid>
  );
};

export default Builds;
