import { Update as UpdateType } from "@monitor/types";
import { Component } from "solid-js";
import Grid from "../../util/layout/Grid";
import s from "../deployment.module.css";

const Update: Component<{ update: UpdateType }> = (p) => {
	return (
		<Grid class={s.Update}>
			
		</Grid>
	);
}

export default Update;