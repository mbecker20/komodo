import { A } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { combineClasses, deploymentStateClass, getId } from "../../../util/helpers";
import Circle from "../../shared/Circle";
import Flex from "../../shared/layout/Flex";
import s from "../home.module.scss";

const Deployment: Component<{ id: string }> = (p) => {
  const { deployments } = useAppState();
  const deployment = () => deployments.get(p.id)!;
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
