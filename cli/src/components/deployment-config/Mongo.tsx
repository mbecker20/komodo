import React from "react";
import { Box, Newline, Text } from "ink";
import { useConfig, useMainSequence } from "../../cli";
import YesNo from "../util/YesNo";
import DeploymentConfig from "./DeploymentConfig";
import EnterToContinue from "../util/EnterToContinue";
import { DEFAULT_MONGO_URL } from "../../config";
import { useEsc, useStore } from "../../util/hooks";
import { Input } from "../util/Input";

type State = {
  setup?: boolean;
  mongoUrl: string;
  confirm: boolean;
}

const Mongo = () => {
  const { set } = useConfig();
  const { next, prev } = useMainSequence();
  const [state, setState, setMany] = useStore<State>({
    mongoUrl: DEFAULT_MONGO_URL,
    confirm: false,
  })

  const { setup, mongoUrl, confirm } = state;

	useEsc(() => {
		if (setup === false) {
      if (confirm) {
        setState("confirm", false);
      } else {
        setState("setup", undefined);
      }
    } else if (setup === undefined) {
      prev();
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
				onSelect={(res) => setState("setup", res === "yes")}
        vertical
      />
    );
  }

  if (setup) {
    return (
      <DeploymentConfig
        deployment="mongo-db"
        back={() => setState("setup", undefined)}
        onFinish={({ name, port, volume, restart }) => {
          set("mongo", {
            url: `mongodb://127.0.0.1:${port}/monitor`,
            startConfig: {
              name,
              port: port as number,
              volume: volume as string | false,
              restart: restart as string,
            },
          });
          next();
        }}
      />
    );
  } else {
    if (confirm) {
      return (
        <Box flexDirection="column">
          <Text color="green">
            mongo url: <Text color="white">{mongoUrl}</Text>
          </Text>
          <Newline />
          <EnterToContinue
            onEnter={() => {
              set("mongo", { url: mongoUrl });
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
            <Input
              initialValue={mongoUrl}
              onSubmit={(mongoUrl) => {
                setMany(["mongoUrl", mongoUrl], ["confirm", true]);
              }}
            />
          </Text>
        </Text>
      );
		}
  }
};

export default Mongo;
