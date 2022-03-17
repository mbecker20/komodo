import { Box, Newline, Text } from "ink";
import React from "react";
import { SetConfig } from "../types";
import Selector from "./util/Selector";

const Builds = (p: { setConfig: SetConfig; next: () => void }) => {
	return (
		<Box flexDirection="column">
			<Text>Would you like to enable docker build services?</Text>
			<Text>
				This will create a{" "}
				<Text color="blue" bold>
					docker registry
				</Text>{" "}
				and enable Monitor to act as a{" "}
				<Text color="blue" bold>
					build server
				</Text>
				.
			</Text>
			<Newline />
			<Selector
				items={["yes", "no"]}
				onSelect={(item) => {
					p.setConfig("useBuilds", item === "yes");
					p.next();
				}}
			/>
		</Box>
	);
};

export default Builds;
