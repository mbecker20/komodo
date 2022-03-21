import { Component } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import s from "./Deployment.module.css";

const Deployment: Component<{}> = (p) => {
	const { servers, deployments } = useAppState();
	return (
		<div class={s.Deployment} >
			
		</div>
	);
}

export default Deployment;