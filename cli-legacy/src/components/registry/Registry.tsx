import { Box, Newline, Text, useInput } from "ink";
import React, { useState } from "react";
import { useBlinker } from "../../hooks";
import { SetConfig } from "../../types";
import YesNo from "../util/YesNo";
import SetupRegistry from "./SetupRegistry";

const Registry = ({
  setConfig,
  next,
}: {
  setConfig: SetConfig;
  next: () => void;
}) => {
  const [setup, setSetup] = useState<boolean>();
  const [registryURL, setRegistryURL] = useState("http://127.0.0.1:5000/");
  const [confirm, setConfirm] = useState(false);
  const blinker = useBlinker();

  useInput((input, key) => {
    if (setup === false) {
      if (key.return) {
        if (confirm) {
          setConfig("registryURL", registryURL);
          next();
        } else {
          setConfirm(true);
        }
      } else if (!confirm && key.delete) {
        setRegistryURL(registryURL.slice(0, registryURL.length - 1));
      } else if (key.leftArrow) {
        setConfirm(false);
      } else if (!confirm) {
        setRegistryURL(registryURL + input);
      }
    }
  });

  if (setup === undefined) {
    return (
      <YesNo
        label={
          <Text>
            Do you need to set up a docker registry locally? This will begin the{" "}
            <Text color="cyan" bold>
              Registry Setup Helper
            </Text>
            .
          </Text>
        }
        onYes={() => {
          setSetup(true);
        }}
        onNo={() => {
          setSetup(false);
        }}
        labelColor="white"
        direction="vertical"
      />
    );
  } else if (setup) {
    return (
      <SetupRegistry
        blinker={blinker}
        onFinished={(registryURL) => {
          setConfig("registryURL", registryURL);
          next();
        }}
      />
    );
  } else {
    return (
      <Box flexDirection="column">
        <Box flexDirection="row">
          <Text color="green">registry url: </Text>
          <Text>
            {registryURL}
            {blinker && !confirm ? "|" : ""}
          </Text>
        </Box>
        <Newline />
        {confirm && (
          <Text>
            press{" "}
            <Text color="green" bold>
              enter
            </Text>{" "}
            to confirm registry url or press the{" "}
            <Text color="blue" bold>
              left arrow
            </Text>{" "}
            on your keyboard to continue editing.
          </Text>
        )}
      </Box>
    );
  }
};

export default Registry;
