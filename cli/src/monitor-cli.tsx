import React from "react";
import { render } from "ink";
// import meow from "meow";
import App from "./App";
import { checkDockerNotInstalled } from "./helpers/docker";

// const cli = meow(
// 	`
// 	Usage
// 	  $ monitor-cli

// 	Options
// 		--name  Your name

// 	Examples
// 	  $ cli --name=Jane
// 	  Hello, Jane
// `,
// 	{
// 		flags: {
// 			name: {
// 				type: "string",
// 			},
// 		},
// 	}
// );
export let dockerNotInstalled = true;
checkDockerNotInstalled().then((res) => {
	dockerNotInstalled = res;
	render(<App />);
});