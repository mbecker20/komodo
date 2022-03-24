import { Component } from "solid-js";
import s from "../deployment.module.css";

const ErrorLog: Component<{}> = (p) => {
	return (
		<div class={s.ErrorLog} >
			error log
		</div>
	);
}

export default ErrorLog;