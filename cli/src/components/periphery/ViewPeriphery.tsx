import React from "react";
import { Box, Text } from "ink";
import { PeripheryConfig } from "../../types";

const ViewPeriphery = ({ config: { name, hostNetwork, port } }: { config: PeripheryConfig }) => {
	return (
    <Box flexDirection="column" marginLeft={2}>
      <Text color="green">
        name: <Text color="white">{name}</Text>
      </Text>
      <Text color="green">
        use host network:{" "}
        <Text color="white">{hostNetwork ? "yes" : "no"}</Text>
      </Text>
      <Text color="green">
        port: <Text color="white">{port}</Text>
      </Text>
    </Box>
  );
}

export default ViewPeriphery;