import { A } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import { PermissionLevel } from "../../../types";
import { combineClasses, deploymentStateClass, getId } from "../../../util/helpers";
import Circle from "../../shared/Circle";
import Icon from "../../shared/Icon";
import Flex from "../../shared/layout/Flex";
import HoverMenu from "../../shared/menu/HoverMenu";
import s from "../home.module.scss";

const Deployment: Component<{ id: string }> = (p) => {
  const { deployments } = useAppState();
  const { user } = useUser();
  const deployment = () => deployments.get(p.id)!;
  const permissionLevel = () => {
    const level = deployment().deployment.permissions![getId(user())];
    return level ? level : PermissionLevel.None;
  };
  return (
    <Show when={deployment()}>
      <A
        href={`/deployment/${p.id}`}
        class={combineClasses(
          s.DropdownItem,
        )}
      >
        <h2>{deployment().deployment.name}</h2>
        <Flex alignItems="center">
          <Show
            when={
              !user().admin && permissionLevel() !== PermissionLevel.None
            }
          >
            <HoverMenu
              target={<Icon type="edit" style={{ padding: "0.25rem" }} />}
              content="you are a collaborator"
              padding="0.5rem"
              position="bottom center"
            />
          </Show>
          <div style={{ opacity: 0.7 }}>{deployments.status(p.id)}</div>
          <Circle
            size={1}
            class={deploymentStateClass(deployments.state(p.id))}
          />
        </Flex>
      </A>
    </Show>
  );
};

export default Deployment;
