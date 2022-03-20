import React, { Fragment, useEffect, useState } from "react";
import { Box, Newline, Text } from "ink";
import { useConfig } from "../cli";
import deploy, { Stage, Update } from "../util/helpers/deploy";

const Setup = () => {
  const { config } = useConfig();
  const [updates, setUpdates] = useState<Update[]>([]);

  useEffect(() => {
    deploy(config, (update) =>
      setUpdates((updates) => {
        const newUpdates = [update, getNextUpdate(update)].filter(
          (val) => val
        ) as Update[];
        return [...updates, ...newUpdates];
      })
    );
  }, []);

  return (
    <Box flexDirection="column">
      <Text>setting up...</Text>
      <Newline />
      {updates.map(({ stage, result, description }) => (
        <Fragment>
          <Text>
            {description} -{" "}
            <Text color="gray">
              ({getStageNumber(stage)} of {config.core ? 4 : 1})
            </Text>
          </Text>
          {result && (
            <Fragment>
              <Newline />
              <Text color="green">
                command: <Text color="white">{result.command}</Text>
              </Text>
              {result.log.stdout ? (
                <Text color="green">
                  stdout: <Text color="white">{result.log.stdout}</Text>
                </Text>
              ) : undefined}
              {result.log.stderr ? (
                <Text color="red">
                  stderr: <Text color="white">{result.log.stderr}</Text>
                </Text>
              ) : undefined}
              <Newline />
            </Fragment>
          )}
        </Fragment>
      ))}
    </Box>
  );
};

function getNextUpdate({ stage }: Update): Update | undefined {
  switch (stage) {
    case "mongo":
      return {
        stage: "periphery",
        description: "",
      };

    case "periphery":
      return {
        stage: "core",
        description: "",
      };

    case "core":
      return {
        stage: "docs",
        description: "",
      };
  }
}

function getStageNumber(stage: Stage) {
  switch (stage) {
    case "mongo":
      return 1;
    case "periphery":
      return 2;
    case "core":
      return 3;
    case "docs":
      return 4;
    case "periphery":
      return 1;
  }
}

export default Setup;
