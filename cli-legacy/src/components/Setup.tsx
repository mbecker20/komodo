import React from "react";
import { Box, Text } from "ink";

const Setup = (p: { periphery: boolean | undefined }) => {
  return (
    <Box flexDirection="column">
      <Text>
        setting up{" "}
        <Text color={p.periphery ? "red" : "cyan"}>
          {p.periphery ? "a peripheral client..." : "monitor core"}
        </Text>{" "}
        {!p.periphery && "using the specified configuration..."}
      </Text>
    </Box>
  );
};

export default Setup;
