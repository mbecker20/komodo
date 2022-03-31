import React from "react";
import { Box, Text } from "ink";
import { CoreOrPeripheryConfig } from "../../types";

const ViewCoreOrPeriphery = ({
  config: { name, secretVolume, port, restart, sysroot },
}: {
  config: CoreOrPeripheryConfig;
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
        system root: <Text color="white">{sysroot}</Text>
      </Text>
      <Text color="green">
        port: <Text color="white">{port}</Text>
      </Text>
      <Text color="green">
        restart: <Text color="white">{restart}</Text>
      </Text>
    </Box>
  );
};

export default ViewCoreOrPeriphery;
