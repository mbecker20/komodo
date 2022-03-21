import React, { Fragment } from "react";
import { Box, Newline, Text } from "ink";
import { useConfig, useMainSequence } from "../../cli";
import { useEsc, useStore } from "../../util/hooks";
import YesNo from "../util/YesNo";
import { DEFAULT_PORT } from "../../config";
import EnterToContinue from "../util/EnterToContinue";
import { ControlledInput } from "../util/Input";
import NumberInput from "../util/NumberInput";
import { CoreOrPeripheryConfig } from "../../types";
import LabelledSelector from "../util/LabelledSelector";
import { toDashedName } from "../../util/helpers/general";

type Stage = "name" | "secret" | "network" | "port" | "restart" | "confirm";

const RESTART_MODES = [
  "always",
  "on failure",
  "unless stopped",
  "don't restart",
];

const CoreOrPeriphery = ({ type }: { type: "core" | "periphery" }) => {
  const { set } = useConfig();
  const { next, prev } = useMainSequence();
  const isCore = type === "core";
  const [config, setConfig, setMany] = useStore<
    Partial<CoreOrPeripheryConfig> & { stage: Stage }
  >({
    stage: "name",
    name: isCore ? "monitor-core" : "monitor-periphery",
  });
  const { stage, name, secretVolume, hostNetwork, port, restart } = config;
  useEsc(() => {
    switch (stage) {
      case "name":
        prev();
        break;

      case "secret":
        setConfig("stage", "name");
        break;

      case "network":
        setConfig("stage", "secret");
        break;

      case "port":
        setMany(
          ["stage", "network"],
          ["hostNetwork", undefined],
          ["port", undefined]
        );
        break;

      case "restart":
        setMany(["stage", "port"]);
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
            <ControlledInput
              value={name!}
              onChange={(name) => setConfig("name", name)}
              onSubmit={(name) => {
                setMany(["stage", "secret"], ["name", name]);
              }}
            />
          ) : (
            name
          )}
        </Text>
      </Text>

      {stage === "secret" && (
        <Text color="green">
          secrets folder:{" "}
          <Text color="white">
            <ControlledInput
              value={secretVolume || "~/secrets"}
              onChange={(volume) => setConfig("secretVolume", volume)}
              onSubmit={(volume) => {
                setMany(["stage", "network"], ["secretVolume", volume]);
              }}
            />
          </Text>
        </Text>
      )}

      {secretVolume && stage !== "secret" && (
        <Text color="green">
          secrets folder: <Text color="white">{secretVolume}</Text>
        </Text>
      )}

      {stage === "network" && hostNetwork === undefined && (
        <YesNo
          label="use host network: "
          onSelect={(res) => {
            setMany(["stage", "port"], ["hostNetwork", res === "yes"]);
          }}
        />
      )}

      {hostNetwork !== undefined && (
        <Text color="green">
          use host network:{" "}
          <Text color="white">{hostNetwork ? "yes" : "no"}</Text>
        </Text>
      )}

      {stage === "port" && (
        <Text color="green">
          port:{" "}
          <Text color="white">
            <NumberInput
              initialValue={port || DEFAULT_PORT}
              onSubmit={(port) => {
                setMany(["stage", "restart"], ["port", port]);
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
          <EnterToContinue
            onEnter={() => {
              set(type, {
                name: name!,
                secretVolume: secretVolume!,
                hostNetwork: hostNetwork!,
                port: Number(port),
                restart: restart!,
              });
              next();
            }}
          />
        </Fragment>
      )}
    </Box>
  );
};

export default CoreOrPeriphery;
