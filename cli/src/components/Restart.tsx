import React, { Fragment, useEffect } from "react";
import LabelledSelector from "./util/LabelledSelector";
import { useMainSequence } from "../cli";
import { useStore } from "../util/hooks";
import { Box, Newline, Text } from "ink";
import { Input } from "./util/Input";
import EnterToContinue from "./util/EnterToContinue";
import { CommandLogError } from "@monitor/types";
import { restart, RestartError } from "../util/helpers/restart";

type State = {
  stage:
    | "query"
    | "name"
    | "mongo"
    | "confirm"
    | "installing"
    | "finished"
    | "error";
  name: string;
  mongoUrl?: string;
  result?: CommandLogError;
  error?: RestartError;
};

const Restart = () => {
  const { next, prev } = useMainSequence();
  const [config, setConfig, setMany] = useStore<State>({
    stage: "query",
    name: "monitor-core",
  });

  const { stage, name, mongoUrl, result, error } = config;

  useEffect(() => {
    if (stage === "installing") {
      restart({ name, mongoUrl: mongoUrl! }, (err) =>
        setMany(["stage", "error"], ["error", err])
      ).then((success) => {
        if (success) {
          setMany(["stage", "finished"], ["result", success]);
        }
      });
    } else if (stage === "finished" || stage === "error") {
      process.exit();
    }
  }, [stage]);

  if (stage === "query") {
    return (
      <LabelledSelector
        label="What are you trying to do?"
        items={["deploy monitor core", "restart monitor core"]}
        onSelect={(option) => {
          switch (option) {
            case "deploy monitor core":
              next();
              break;

            case "restart monitor core":
              setConfig("stage", "name");
              break;
          }
        }}
        onEsc={prev}
        vertical
      />
    );
  } else {
    return (
      <Box flexDirection="column">
        <Text color="green">
          name:{" "}
          <Text color="white">
            {stage === "name" ? (
              <Input
                initialValue={name}
                onSubmit={(name) => setMany(["stage", "mongo"], ["name", name])}
                onEsc={() => setConfig("stage", "query")}
              />
            ) : (
              name
            )}
          </Text>
        </Text>

        {stage === "mongo" && (
          <Text color="green">
            mongo url:{" "}
            <Text color="white">
              <Input
                initialValue={mongoUrl || "mongodb://127.0.0.1:27017/monitor"}
                onSubmit={(mongoUrl) =>
                  setMany(["stage", "confirm"], ["mongoUrl", mongoUrl])
                }
                onEsc={() => setConfig("stage", "name")}
              />
            </Text>
          </Text>
        )}

        {mongoUrl && stage !== "mongo" && (
          <Text color="green">
            mongo url: <Text color="white">{mongoUrl}</Text>
          </Text>
        )}

        <Newline />

        {stage === "confirm" && (
          <EnterToContinue
            onEnter={() => {
              setConfig("stage", "installing");
            }}
            onEsc={() => setConfig("stage", "mongo")}
            pressEnterTo="restart monitor"
          />
        )}

        {(stage === "installing" || stage === "error") && (
          <Fragment>
            <Text>restarting...</Text>
          </Fragment>
        )}

        {result && (
          <Fragment>
            <Text color="green">finished restarting</Text>
            <Newline />
            <Box flexDirection="column" marginLeft={2}>
              <Text color="green">
                command: <Text color="white">{result.command}</Text>
              </Text>
              {result.log.stderr ? (
                <Text color="red">
                  stderr: <Text color="white">{result.log.stderr}</Text>
                </Text>
              ) : undefined}
              {result.log.stdout ? (
                <Text color="blue">
                  stdout: <Text color="white">{result.log.stdout}</Text>
                </Text>
              ) : undefined}
            </Box>
            <Newline />
          </Fragment>
        )}

        {error && (
          <Fragment>
            <Newline />
            <Text color="red">error restarting</Text>
            <Newline />
            <Text>{error.message}</Text>
            <Text>{error.error}</Text>
            <Newline />
          </Fragment>
        )}
      </Box>
    );
  }
};

export default Restart;
