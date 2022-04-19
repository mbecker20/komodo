import React, { Fragment, useEffect } from "react";
import LabelledSelector from "./util/LabelledSelector";
import { useMainSequence } from "../cli";
import { useStore } from "../util/hooks";
import { Box, Newline, Text } from "ink";
import { Input } from "./util/Input";
import EnterToContinue from "./util/EnterToContinue";
import { CommandLogError } from "@monitor/types";
import { restart, RestartError } from "../util/helpers/restart";
import YesNo from "./util/YesNo";

type State = {
  stage:
    | "query"
    | "mongo"
    | "pullLatest"
    | "confirm"
    | "installing"
    | "finished"
    | "error";
  pullLatest?: boolean;
  mongoUrl?: string;
  result?: CommandLogError;
  error?: RestartError;
};

const DEPLOY_CORE_OR_PERIPHERY = "deploy monitor core or periphery";
const RESTART_CORE = "restart monitor core";

const Restart = ({
  useDefaults,
  defaultMongoUrl,
  defaultPullLatest,
}: {
  useDefaults?: boolean;
  defaultMongoUrl?: string;
  defaultPullLatest?: boolean;
}) => {
  const { next, prev } = useMainSequence();
  const [config, setConfig, setMany] = useStore<State>({
    stage:
      useDefaults || defaultMongoUrl
        ? "installing"
        : defaultMongoUrl
        ? "pullLatest"
        : "query",
    mongoUrl: useDefaults
      ? "mongodb://127.0.0.1:27017/monitor"
      : defaultMongoUrl,
    pullLatest: useDefaults ? false : defaultPullLatest,
  });

  const { stage, mongoUrl, pullLatest, result, error } = config;

  useEffect(() => {
    if (stage === "installing") {
      restart({ mongoUrl: mongoUrl!, pullLatest: pullLatest! }, (err) =>
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
        items={[DEPLOY_CORE_OR_PERIPHERY, RESTART_CORE]}
        onSelect={(option) => {
          switch (option) {
            case DEPLOY_CORE_OR_PERIPHERY:
              next();
              break;

            case RESTART_CORE:
              setConfig("stage", "mongo");
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
        {stage === "mongo" && (
          <Text color="green">
            mongo url:{" "}
            <Text color="white">
              <Input
                initialValue={mongoUrl || "mongodb://127.0.0.1:27017/monitor"}
                onSubmit={(mongoUrl) =>
                  setMany(["stage", "pullLatest"], ["mongoUrl", mongoUrl])
                }
                onEsc={() => setConfig("stage", "query")}
              />
            </Text>
          </Text>
        )}

        {mongoUrl && stage !== "mongo" && (
          <Text color="green">
            mongo url: <Text color="white">{mongoUrl}</Text>
          </Text>
        )}

        {stage === "pullLatest" && (
          <YesNo
            label="pull latest core?"
            onSelect={(res) => {
              setMany(["stage", "confirm"], ["pullLatest", res === "yes"]);
            }}
            onEsc={() => setConfig("stage", "mongo")}
          />
        )}

        {pullLatest !== undefined && stage !== "pullLatest" && (
          <Text color="green">
            pull latest: <Text color="white">{pullLatest ? "yes" : "no"}</Text>
          </Text>
        )}

        <Newline />

        {stage === "confirm" && (
          <EnterToContinue
            onEnter={() => {
              setConfig("stage", "installing");
            }}
            onEsc={() => setConfig("stage", "pullLatest")}
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
