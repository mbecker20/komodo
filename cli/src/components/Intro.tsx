import React from "react";
import { Box, Newline, Text } from "ink";
import EnterToContinue from "./util/EnterToContinue";

const Intro = ({ next }: { next: () => void }) => {
  return (
    <Box flexDirection="column">
      <Text>
        this is a CLI to setup{" "}
        <Text color="cyan" bold>
          monitor
        </Text>
        , a tool to manage application deployment.
      </Text>
      <Newline />
      <EnterToContinue onEnter={next} />
    </Box>
  );
};

export default Intro;
