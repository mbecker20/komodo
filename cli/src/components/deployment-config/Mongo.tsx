import React, { useState } from "react";
import { Box, Newline, Text } from "ink";
import TextInput from "ink-text-input";
import { useConfig, useMainSequence } from "../../cli";
import YesNo from "../util/YesNo";
import DeploymentConfig from "./DeploymentConfig";
import EnterToContinue from "../util/EnterToContinue";
import { DEFAULT_MONGO_URL } from "../../config";
import { useEsc } from "../../util/hooks";

const Mongo = () => {
  const { set } = useConfig();
  const { next } = useMainSequence();
  const [setup, setSetup] = useState<boolean>();
  const [mongoURL, setMongoURL] = useState(DEFAULT_MONGO_URL);
  const [confirm, setConfirm] = useState(false);

	useEsc(() => {
		if (!setup && confirm) {
			setConfirm(false);
		}
	})

  if (setup === undefined) {
    return (
      <YesNo
        label={
          <Text>
            do you need to set up{" "}
            <Text color="cyan" bold>
              mongo db
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
          mongo db config
        </Text>
        <Newline />
        <DeploymentConfig
          deployment="mongo-db"
          onFinish={({ name, port, volume, restart }) => {
            set("mongo", {
              url: `mongodb://127.0.0.1:${port}/monitor`,
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
            mongo url: <Text color="white">{mongoURL}</Text>
          </Text>
          <Newline />
          <EnterToContinue
            onEnter={() => {
              set("mongo", { url: mongoURL });
              next();
            }}
          />
        </Box>
      );
    } else {
			return (
        <Text color="green">
          mongo url:{" "}
          <Text color="white">
            <TextInput
              value={mongoURL}
              onChange={setMongoURL}
              onSubmit={() => setConfirm(true)}
            />
          </Text>
        </Text>
      );
		}
  }
};

export default Mongo;
