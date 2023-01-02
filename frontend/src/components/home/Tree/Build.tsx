import { A } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import { PermissionLevel } from "../../../types";
import { combineClasses, getId } from "../../../util/helpers";
import Icon from "../../shared/Icon";
import HoverMenu from "../../shared/menu/HoverMenu";
import s from "../home.module.scss";

const Build: Component<{ id: string }> = (p) => {
  const { builds } = useAppState();
  const { user } = useUser();
  const build = () => builds.get(p.id)!;
  const permissionLevel = () => {
    const level = build().permissions[getId(user())];
    return level ? level : PermissionLevel.None;
  };
  return (
    <Show when={build()}>
      <A href={`/build/${p.id}`} class={combineClasses(s.DropdownItem)}>
        <div>{build().name}</div>
        <Show
          when={!user().admin && permissionLevel() !== PermissionLevel.None}
        >
          <HoverMenu
            target={<Icon type="edit" style={{ padding: "0.25rem" }} />}
            content="you are a collaborator"
            padding="0.5rem"
            position="bottom right"
          />
        </Show>
      </A>
    </Show>
  );
};
export default Build;
