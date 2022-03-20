import React, { Fragment } from "react";
import { Box, Newline, Text } from "ink";
import TextInput from "ink-text-input";
import { useConfig, useMainSequence } from "../../cli";
import { useStore } from "../../util/hooks";
import { CoreConfig } from "../../types";
import YesNo from "../util/YesNo";
import { DEFAULT_PORT } from "../../config";
import EnterToContinue from "../util/EnterToContinue";

type Stage = "name" | "secret" | "network" | "port" | "confirm";

const Core = () => {
  const { set } = useConfig();
  const { next, prev } = useMainSequence();
  const [config, setConfig, setMany] = useStore<
    Partial<CoreConfig> & { stage: Stage }
  >({
    stage: "name",
    name: "monitor-core",
  });
  const { stage, name, secretVolume, hostNetwork, port } = config;
  return (
    <Box flexDirection="column">
      <Text color="cyan" bold>
        monitor core config
      </Text>
      <Newline />

      <Text color="green">
        name:{" "}
        <Text color="white">
          {stage === "name" ? (
            <TextInput
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
            <TextInput
              value={secretVolume || "~/secrets"}
              onChange={(volume) => setConfig("secretVolume", volume)}
              onSubmit={(volume) => {
                setMany(["stage", "network"], ["secretVolume", volume]);
              }}
            />
          </Text>
        </Text>
      )}

      {secretVolume && (
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
          noYes
        />
      )}

      {hostNetwork !== undefined && (
        <Text color="green">
          use host network:{" "}
          <Text color="white">{hostNetwork ? "yes" : "no"}</Text>
        </Text>
      )}

      {stage === "port" && port === undefined && (
        <Text color="green">
          port:{" "}
          <Text color="white">
            <TextInput
              value={port || DEFAULT_PORT.toString()}
              onChange={(port) => {
                setConfig("port", port);
              }}
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
              set("core", {
                name: name as string,
                secretVolume: secretVolume as string,
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

export default Core;
