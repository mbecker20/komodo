import React, { Fragment, useState } from "react";
import { Box, Newline, Text } from "ink";
import TextInput from "ink-text-input";
import { useEsc, useStore } from "../../util/hooks";
import { useMainSequence } from "../../cli";
import EnterToContinue from "../util/EnterToContinue";
import LabelledSelector from "../util/LabelledSelector";
import YesNo from "../util/YesNo";

type DeploymentConfig = {
  stage: "name" | "port" | "volume" | "restart" | "confirm";
  name: string;
  port?: string;
  volume?: string | false;
  restart?: string;
};

const RESTART_MODES = ["no", "on-failure", "always", "unless-stopped"];

const DeploymentConfig = ({
  deployment,
  onFinish,
}: {
  deployment: "mongo-db" | "registry";
  onFinish: (config: DeploymentConfig) => void;
}) => {
  const [config, setConfig, setMany] = useStore<DeploymentConfig>({
    stage: "name",
    name: deployment,
  });
  const { stage, name, port, volume, restart } = config;
  const { next } = useMainSequence();
  useEsc(() => {
    switch (stage) {
      case "port":
        setConfig("stage", "name");
        break;

      case "volume":
        setMany(["stage", "port"], ["volume", undefined]);
        break;

      case "restart":
        setMany(["stage", "volume"], ["volume", undefined]);
        break;

      case "confirm":
        setMany(["stage", "restart"], ["restart", undefined]);
        break;
    }
  });
  return (
    <Box flexDirection="column">
      <Text color="green">
        name:{" "}
        <Text color="white">
          {stage === "name" ? (
            <TextInput
              value={name}
              onChange={(name) => setConfig("name", name)}
              onSubmit={(name) => {
                // setConfig("port", deployment === "mongo-db" ? "27017" : "5000");
                setMany(["name", name], ["stage", "port"]);
              }}
            />
          ) : (
            name
          )}
        </Text>
      </Text>

      {stage === "port" && (
        <Text color="green">
          port:{" "}
          <Text color="white">
            <TextInput
              value={port || (deployment === "mongo-db" ? "27017" : "5000")}
              onChange={(port) => setConfig("port", port)}
              onSubmit={(port) => {
                setMany(["stage", "volume"], ["port", port]);
              }}
            />
          </Text>
        </Text>
      )}

      {port && stage !== "port" && (
        <Text color="green">
          port: <Text color="white">{port}</Text>
        </Text>
      )}

      {stage === "volume" && volume === undefined && (
        <YesNo
          label={
            <Text>
              mount data on local filesystem? this is used to{" "}
              <Text color="green">persist data</Text> between{" "}
              <Text color="green">container restarts</Text>.
            </Text>
          }
          onSelect={(use) => {
            if (use === "yes") {
              setConfig("volume", `~/${name}`);
            } else {
              setMany(["stage", "restart"], ["volume", false]);
            }
          }}
          vertical
        />
      )}

      {volume !== undefined && (
        <Text color="green">
          mount folder:{" "}
          <Text color="white">
            {stage === "volume" ? (
              <TextInput
                value={volume as string}
                onChange={(volume) => setConfig("volume", volume)}
                onSubmit={(volume) => {
                  setMany(["stage", "restart"], ["volume", volume]);
                }}
              />
            ) : (
              volume || "no"
            )}
          </Text>
        </Text>
      )}

      {stage === "restart" && (
        <LabelledSelector
          label="restart: "
          items={RESTART_MODES}
          onSelect={(restart) => {
            setMany(["stage", "confirm"], ["restart", restart]);
          }}
        />
      )}

      {restart && (
        <Text color="green">
          restart: <Text color="white">{restart}</Text>
        </Text>
      )}

      {stage === "confirm" && (
        <Fragment>
          <Newline />
          <EnterToContinue onEnter={() => onFinish(config)} />
        </Fragment>
      )}
    </Box>
  );
};

export default DeploymentConfig;
