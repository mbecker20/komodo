import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import s from "../sidebar.module.css";

const Deployment: Component<{ id: string }> = (p) => {
	const { deployments } = useAppState();
	const deployment = () => deployments.get(p.id);
	
	return (
    <Show when={deployment()}>
      <div class={s.Deployment}>
				<div>{deployment()!.name}</div>
			</div>
    </Show>
  );
}

export default Deployment;