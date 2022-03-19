import React from "react";
import { Box, Newline, Text } from "ink";
import EnterToContinue from "./util/EnterToContinue";
import { useMainSequence } from "../cli";

const Intro = () => {
  const { next } = useMainSequence();
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
