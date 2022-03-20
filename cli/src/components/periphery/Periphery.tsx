import React, { Fragment } from "react";
import { Box, Newline, Text } from "ink";
import { useConfig, useMainSequence } from "../../cli";
import { useEsc, useStore } from "../../util/hooks";
import { PeripheryConfig } from "../../types";
import YesNo from "../util/YesNo";
import { DEFAULT_PORT } from "../../config";
import EnterToContinue from "../util/EnterToContinue";
import { ControlledInput } from "../util/Input";
import NumberInput from "../util/NumberInput";

type Stage = "name" | "network" | "port" | "confirm";

const Periphery = () => {
  const { set } = useConfig();
  const { next, prev } = useMainSequence();
  const [config, setConfig, setMany] = useStore<
    Partial<PeripheryConfig> & { stage: Stage }
  >({
    stage: "name",
    name: "monitor-periphery",
  });
  const { stage, name, hostNetwork, port } = config;
  useEsc(() => {
    switch (stage) {
      case "name":
        prev();
        break;

      case "network":
        setConfig("stage", "name");
        break;

      case "port":
        setMany(
          ["stage", "network"],
          ["hostNetwork", undefined],
          ["port", undefined]
        );
        break;

      case "confirm":
        setMany(["stage", "port"], ["port", undefined]);
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
                setMany(["stage", "network"], ["name", name]);
              }}
            />
          ) : (
            name
          )}
        </Text>
      </Text>

      {stage === "network" && hostNetwork === undefined && (
        <YesNo
          label="use host network: "
          onSelect={(res) => {
            setMany(["stage", "port"], ["hostNetwork", res === "yes"]);
          }}
          noYes
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
                setMany(["stage", "confirm"], ["port", port]);
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

      {stage === "confirm" && (
        <Fragment>
          <Newline />
          <EnterToContinue
            onEnter={() => {
              set("periphery", {
                name: name as string,
                hostNetwork: hostNetwork as boolean,
                port: Number(port),
              });
              next();
            }}
          />
        </Fragment>
      )}
    </Box>
  );
};

export default Periphery;
