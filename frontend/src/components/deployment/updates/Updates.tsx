import { Component } from "solid-js";
import { useUpdates } from "../../../state/hooks";
import { useAppState } from "../../../state/StateProvider";
import s from "../deployment.module.css";

const Updates: Component<{ deploymentID: string }> = (p) => {
	const { updates, ws } = useAppState();
	const selectedUpdates = useUpdates({ deploymentID: p.deploymentID });
	
	return (
		<div class={s.Updates} >
			
		</div>
	);
}

export default Updates;