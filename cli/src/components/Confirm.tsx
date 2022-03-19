import React from "react";
import { useConfig, useMainSequence } from "../cli";
import EnterToContinue from "./util/EnterToContinue";
import { useEsc } from "../util/hooks";
import { Box, Newline, Text } from "ink";
import View from "./deployment-config/View";

const Confirm = ({ next }: { next: () => void }) => {
  const { config } = useConfig();
  const { prev } = useMainSequence();
  useEsc(prev);
  return (
    <Box flexDirection="column">
      <Text color="cyan" bold>
        mongo:
      </Text>
      <View url={config.mongo!.url} config={config.mongo?.startConfig} />
      <Newline />

      <Text color="cyan" bold>
        registry:
      </Text>
      <View url={config.registry!.url!} config={config.registry?.startConfig} />
      <Newline />

      <EnterToContinue pressEnterTo="install" onEnter={next} />
    </Box>
  );
};

export default Confirm;
