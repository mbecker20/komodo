import { ContainerStatus } from "@monitor/types";
import { Component, Show } from "solid-js";
import { useAppDimensions } from "../../../state/DimensionProvider";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import { combineClasses, deploymentStatusClass } from "../../../util/helpers";
import Icon from "../../util/Icon";
import Flex from "../../util/layout/Flex";
import HoverMenu from "../../util/menu/HoverMenu";
import s from "../sidebar.module.scss";

const Deployment: Component<{ id: string }> = (p) => {
  const { deployments, selected, sidebar } = useAppState();
  const { width } = useAppDimensions();
  const { permissions, username } = useUser();
  const deployment = () => deployments.get(p.id)!;
  const status = () => {
    if (!deployment() || deployment()!.status === "not deployed") {
      return "not deployed";
    } else {
      return (deployment()!.status as ContainerStatus).State;
    }
  };
  return (
    <Show when={deployment()}>
      <button
        class={combineClasses(
          s.DropdownItem,
          selected.id() === p.id && "selected"
        )}
        onClick={() => {
          selected.set(deployment()!._id!, "deployment");
          if (width() <= 1000) {
            sidebar.toggle();
          }
        }}
      >
        <div>{deployment()!.name}</div>
        <Flex alignItems="center">
          <Show
            when={
              permissions() === 1 && deployment().owners.includes(username()!)
            }
          >
            <HoverMenu
              target={<Icon type="edit" style={{ padding: "0.25rem" }} />}
              content="you are a collaborator"
              padding="0.5rem"
              position="bottom center"
            />
          </Show>
          <div class={deploymentStatusClass(status())}>{status()}</div>
        </Flex>
      </button>
    </Show>
  );
};

export default Deployment;
