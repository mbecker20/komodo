import React from "react";
import { Box, Newline, Text } from "ink";
import Selector from "./util/Selector";
import { SetConfig } from "../types";

const Periphery = (p: {
  setPeriphery: (a: boolean) => void;
  next: () => void;
}) => {
  return (
    <Box flexDirection="column">
      <Text>
        Are you setting up <Text color="cyan">monitor core</Text> or {" "}
        <Text color="red">monitor periphery</Text>?
      </Text>
      <Newline />
      <Selector
        items={["core", "peripheral"]}
        onSelect={(item) => {
          p.setPeriphery(item === "periphery");
          p.next();
        }}
      />
    </Box>
  );
};

export default Periphery;
