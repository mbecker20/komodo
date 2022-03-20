import React, { Fragment } from "react";
import { Box, Newline, Text } from "ink";
import { useEsc, useStore } from "../../util/hooks";
import EnterToContinue from "../util/EnterToContinue";
import LabelledSelector from "../util/LabelledSelector";
import YesNo from "../util/YesNo";
import { toDashedName } from "../../util/helpers/general";
import { Input } from "../util/Input";
import NumberInput from "../util/NumberInput";

type DeploymentConfig = {
  stage: "name" | "port" | "volume" | "restart" | "confirm";
  name: string;
  port?: number;
  volume?: string | false;
  restart?: string;
};

const RESTART_MODES = [
  "always",
  "on failure",
  "unless stopped",
  "don't restart",
];

const DeploymentConfig = ({
  deployment,
  onFinish,
  back,
}: {
  deployment: "mongo-db" | "registry";
  onFinish: (config: DeploymentConfig) => void;
  back: () => void;
}) => {
  const [config, setConfig, setMany] = useStore<DeploymentConfig>({
    stage: "name",
    name: deployment,
  });
  const { stage, name, port, volume, restart } = config;
  useEsc(() => {
    switch (stage) {
      case "name":
        back();
        break;

      case "port":
        setConfig("stage", "name");
        break;

      case "volume":
        if (volume) {
          setConfig("volume", undefined);
        } else {
          setMany(["stage", "port"], ["volume", undefined]);
        }
        break;

      case "restart":
        setMany(
          ["stage", "volume"],
          volume === false ? ["volume", undefined] : ["volume", volume]
        );
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
            <Input
              initialValue={name}
              onSubmit={(name) => {
                setMany(["stage", "port"], ["name", name]);
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
            <NumberInput
              initialValue={port || (deployment === "mongo-db" ? 27017 : 5000)}
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
              <Input
                initialValue={volume as string}
                onSubmit={(volume) => {
                  setMany(["stage", "restart"], ["volume", volume]);
                }}
              />
            ) : (
              volume || "don't use"
            )}
          </Text>
        </Text>
      )}

      {stage === "restart" && (
        <LabelledSelector
          label="restart: "
          items={RESTART_MODES}
          onSelect={(restart) => {
            setMany(
              ["stage", "confirm"],
              [
                "restart",
                restart === "don't restart" ? "no" : toDashedName(restart),
              ]
            );
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
