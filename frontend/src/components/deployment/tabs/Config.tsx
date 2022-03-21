import { Component } from "solid-js";
import { useToggle } from "../../../util/hooks";

const Config: Component<{}> = (p) => {
	const [editing, toggleEditing] = useToggle();
	return (
		<div>
			
		</div>
	);
}

export default Config;