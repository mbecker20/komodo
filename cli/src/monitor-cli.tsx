#!/usr/bin/env node
import React from "react";
import { render } from "ink";
// import meow from "meow";
import App from "./App";

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

render(<App />);
