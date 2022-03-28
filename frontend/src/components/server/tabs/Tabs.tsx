import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import s from "../server.module.css";

const ServerTabs: Component<{}> = (p) => {
	const { servers, selected } = useAppState();
  const server = () => servers.get(selected.id())!;
	return (
		<Show when={server()}>
			
		</Show>
	);
}

export default ServerTabs;