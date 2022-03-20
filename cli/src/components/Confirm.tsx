import React, { Fragment } from "react";
import { useConfig, useMainSequence } from "../cli";
import EnterToContinue from "./util/EnterToContinue";
import { useEsc } from "../util/hooks";
import { Box, Newline, Text } from "ink";
import View from "./deployment-config/View";
import ViewCore from "./core/ViewCore";
import ViewPeriphery from "./periphery/ViewPeriphery";

const Confirm = ({ next }: { next: () => void }) => {
  const { config } = useConfig();
  const { prev } = useMainSequence();
  useEsc(prev);
  return (
    <Box flexDirection="column">
      {config.mongo && (
        <Fragment>
          <Text color="cyan" bold>
            mongo:
          </Text>
          <View url={config.mongo.url} config={config.mongo.startConfig} />
          <Newline />
        </Fragment>
      )}

      {config.registry && (
        <Fragment>
          <Text color="cyan" bold>
            registry:
          </Text>
          <View
            url={config.registry.url!}
            config={config.registry.startConfig}
          />
          <Newline />
        </Fragment>
      )}

      {config.core && (
        <Fragment>
          <Text color="cyan" bold>
            monitor core:
          </Text>
          <ViewCore config={config.core} />
          <Newline />
        </Fragment>
      )}

      {config.periphery && (
        <Fragment>
          <Text color="cyan" bold>
            monitor periphery:
          </Text>
          <ViewPeriphery config={config.periphery} />
          <Newline />
        </Fragment>
      )}

      <EnterToContinue pressEnterTo="install" onEnter={next} />
    </Box>
  );
};

export default Confirm;
