import { A } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import { PermissionLevel } from "../../../types";
import {
  combineClasses,
  getId,
  readableMonitorTimestamp,
} from "../../../util/helpers";
import Icon from "../../shared/Icon";
import Flex from "../../shared/layout/Flex";
import HoverMenu from "../../shared/menu/HoverMenu";
import s from "../home.module.scss";

const Build: Component<{ id: string }> = (p) => {
  const { builds } = useAppState();
  const { user } = useUser();
  const build = () => builds.get(p.id)!;
  const permissionLevel = () => {
    const level = build().permissions![getId(user())];
    return level ? level : PermissionLevel.None;
  };
  const version = () => {
    return `v${build().version.major}.${build().version.minor}.${
      build().version.patch
    }`;
  };
  const lastBuiltAt = () => {
    if (
      build().last_built_at === undefined ||
      build().last_built_at?.length === 0 ||
      build().last_built_at === "never"
    ) {
      return "not built";
    } else {
      return readableMonitorTimestamp(build().last_built_at!);
    }
  };
  return (
    <Show when={build()}>
      <A href={`/build/${p.id}`} class={combineClasses(s.DropdownItem)}>
        <h2>{build().name}</h2>
        <Flex>
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
          <div>{version()}</div>
          <div style={{ opacity: 0.7 }}>{lastBuiltAt()}</div>
        </Flex>
      </A>
    </Show>
  );
};
export default Build;
