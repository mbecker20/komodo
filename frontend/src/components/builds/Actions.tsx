import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/icons/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import s from "./build.module.css";

const Actions: Component<{}> = (p) => {
	const { builds, selected } = useAppState();
  const build = () => builds.get(selected.id())!;
	return (
		<Show when={build()}>
			<Grid class={combineClasses(s.Card, "shadow")}>
				<h1>actions</h1>
				<Flex class={combineClasses(s.Action, "shadow")}>
					build{" "}
					<ConfirmButton color="green">
						<Icon type="build" />
					</ConfirmButton>
				</Flex>
			</Grid>
		</Show>
	);
}

export default Actions;