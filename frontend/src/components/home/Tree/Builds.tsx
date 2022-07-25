import { Component, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useTheme } from "../../../state/ThemeProvider";
import { useUser } from "../../../state/UserProvider";
import { combineClasses } from "../../../util/helpers";
import Icon from "../../util/Icon";
import Grid from "../../util/layout/Grid";
import HoverMenu from "../../util/menu/HoverMenu";
import s from "../home.module.scss";
import { NewBuild } from "./New";

const Builds: Component<{}> = (p) => {
	const { builds } = useAppState();
	const { themeClass } = useTheme();
	return (
    <Grid
      class={combineClasses(
        s.Deployments,
        themeClass()
      )}
    >
      <For each={builds.ids()}>{(id) => <Build id={id} />}</For>
      <NewBuild />
    </Grid>
  );
}

const Build: Component<{ id: string }> = (p) => {
  const { builds, selected } = useAppState();
  const { permissions, username } = useUser();
  const build = () => builds.get(p.id)!;
  return (
    <Show when={build()}>
      <button
        class={combineClasses(
          selected.id() === p.id && "selected",
          s.DropdownItem
        )}
        onClick={() => selected.set(build()._id!, "build")}
      >
        <div>{build().name}</div>
        <Show when={permissions() === 1 && build().owners.includes(username())}>
          <HoverMenu
            target={<Icon type="edit" style={{ padding: "0.25rem" }} />}
            content="you are a collaborator"
            padding="0.5rem"
            position="bottom right"
          />
        </Show>
      </button>
    </Show>
  );
};
export default Builds;