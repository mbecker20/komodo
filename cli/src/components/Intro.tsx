import React from "react";
import { Box, Newline, Text, useInput } from "ink";

const Intro = (p: { next: () => void }) => {
	useInput((_, key) => {
		if (key.return) p.next();
	});
	return (
		<Box flexDirection="column">
			<Text>
				This is a CLI to setup{" "}
				<Text color="cyan" bold>
					Monitor
				</Text>
				, a tool to manage application deployment.
			</Text>
			<Newline />
			<Text>
				press{" "}
				<Text color="green" bold>
					enter
				</Text>{" "}
				to continue.
			</Text>
		</Box>
	);
};

export default Intro;
