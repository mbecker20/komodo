import React from "react";
import { Box, Text } from "ink";
import { Config, Next } from "../types";
import { useEnter } from "../hooks";

const Confirm = ({ config, next }: { config: Config; next: Next }) => {
	useEnter(next);
  return <Box><Text>confirm</Text></Box>;
};

export default Confirm;
