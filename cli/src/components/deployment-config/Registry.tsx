import React from "react";
import { Box, Newline, Text } from "ink";
import { useConfig, useMainSequence } from "../../cli";
import YesNo from "../util/YesNo";
import DeploymentConfig from "./DeploymentConfig";
import EnterToContinue from "../util/EnterToContinue";
import { DEFAULT_REGISTRY_URL } from "../../config";
import { useEsc, useStore } from "../../util/hooks";
import { Input } from "../util/Input";
import { toDashedName } from "../../util/helpers/general";

type State = {
  setup?: boolean;
  regUrl: string;
  confirm: boolean;
};

const Registry = () => {
  const { set } = useConfig();
  const { next, prev } = useMainSequence();
  const [state, setState, setMany] = useStore<State>({
    regUrl: DEFAULT_REGISTRY_URL,
    confirm: false,
  });
  const { setup, regUrl, confirm } = state;

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
        onSelect={(res) => setState("setup", res === "yes")}
        vertical
      />
    );
  }

  if (setup) {
    return (
      <DeploymentConfig
        deployment="registry"
        back={() => setState("setup", undefined)}
        onFinish={({ name, port, volume, restart }) => {
          set("registry", {
            url: `http://${toDashedName(name)}:${port}/`,
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
    );
  } else {
    if (confirm) {
      return (
        <Box flexDirection="column">
          <Text color="green">
            registry url: <Text color="white">{regUrl}</Text>
          </Text>
          <Newline />
          <EnterToContinue
            onEnter={() => {
              set("registry", { url: regUrl });
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
            <Input
              initialValue={regUrl}
              onSubmit={(regUrl) => setMany(["regUrl", regUrl], ["confirm", true])}
            />
          </Text>
        </Text>
      );
    }
  }
};

export default Registry;
