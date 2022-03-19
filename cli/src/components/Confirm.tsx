import React from "react";
import { Box, Text } from "ink";
import { useEnter } from "../util/hooks";
import { useConfig } from "../cli";

const Confirm = ({ next }: { next: () => void }) => {
	const { config } = useConfig();
	useEnter(next);
  return <Box><Text>confirm</Text></Box>;
};

export default Confirm;
