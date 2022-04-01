import { ContainerStatus } from "@monitor/types";
import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { combineClasses } from "../../../util/helpers";
import s from "../sidebar.module.css";

const Deployment: Component<{ id: string }> = (p) => {
  const { deployments, selected } = useAppState();
  const deployment = () => deployments.get(p.id);
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
          selected.id() === p.id && "selected",
        )}
        onClick={() => selected.set(deployment()!._id!, "deployment")}
      >
        <div>{deployment()!.name}</div>
        <div>{status()}</div>
      </button>
    </Show>
  );
};

export default Deployment;
