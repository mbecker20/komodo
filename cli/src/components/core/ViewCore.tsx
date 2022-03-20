import React from "react";
import { Box, Text } from "ink";
import { CoreConfig } from "../../types";

const ViewCore = ({
  config: { name, secretVolume, hostNetwork, port },
}: {
  config: CoreConfig;
}) => {
  return (
    <Box flexDirection="column" marginLeft={2}>
      <Text color="green">
        name: <Text color="white">{name}</Text>
      </Text>
      <Text color="green">
        secrets folder: <Text color="white">{secretVolume}</Text>
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
};

export default ViewCore;
