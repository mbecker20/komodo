import React, { useState } from "react";
import { Box, Newline, Text } from "ink";
import TextInput from "ink-text-input";
import { useConfig, useMainSequence } from "../../cli";
import YesNo from "../util/YesNo";
import DeploymentConfig from "./DeploymentConfig";
import EnterToContinue from "../util/EnterToContinue";
import { DEFAULT_REGISTRY_URL } from "../../config";
import { useEsc } from "../../util/hooks";

const Registry = () => {
  const { set } = useConfig();
  const { next } = useMainSequence();
  const [setup, setSetup] = useState<boolean>();
  const [regURL, setRegURL] = useState(DEFAULT_REGISTRY_URL);
  const [confirm, setConfirm] = useState(false);

  useEsc(() => {
    if (!setup && confirm) {
      setConfirm(false);
    }
  });

  if (setup === undefined) {
    return (
      <YesNo
        label={
          <Text>
            do you need to set up a{" "}
            <Text color="cyan" bold>
              docker registry
            </Text>{" "}
            locally?{" "}
          </Text>
        }
        onSelect={(res) => setSetup(res === "yes")}
        vertical
      />
    );
  }

  if (setup) {
    return (
      <Box flexDirection="column">
        <Text color="cyan" bold>
          registry config
        </Text>
        <Newline />
        <DeploymentConfig
          deployment="registry"
          onFinish={({ name, port, volume, restart }) => {
            set("registry", {
              exists: true,
              url: `http://127.0.0.1:${port}/`,
              startConfig: {
                name,
                port: Number(port),
                volume: volume as string | false,
                restart: restart as string,
              },
            });
            next();
          }}
        />
      </Box>
    );
  } else {
    if (confirm) {
      return (
        <Box flexDirection="column">
          <Text color="green">
            registry url: <Text color="white">{regURL}</Text>
          </Text>
          <Newline />
          <EnterToContinue
            onEnter={() => {
              set("registry", { url: regURL });
              next();
            }}
          />
        </Box>
      );
    } else {
      return (
        <Text color="green">
          registry url:{" "}
          <Text color="white">
            <TextInput
              value={regURL}
              onChange={setRegURL}
              onSubmit={() => setConfirm(true)}
            />
          </Text>
        </Text>
      );
    }
  }
};

export default Registry;
