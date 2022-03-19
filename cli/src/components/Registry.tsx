import React from "react";
import { Box } from "ink";
import { useConfig, useMainSequence } from "../cli";
import { createUseSequence } from "../util/state";

const useRegisterySequence = createUseSequence();

const Registry = () => {
	const { prev } = useMainSequence();
	const { current } = useRegisterySequence();
	const { config, set } = useConfig();
	
	return (
		<Box>
			
		</Box>
	);
}

export default Registry;