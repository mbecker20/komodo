import { Component } from "solid-js";
import s from "./home.module.scss"
import Summary from "./Summary";

const Home: Component<{}> = (p) => {
	return (
		<div>
			<Summary />
		</div>
	);
}

export default Home;