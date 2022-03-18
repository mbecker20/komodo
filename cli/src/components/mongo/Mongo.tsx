import { Box, Newline, Text, useInput } from "ink";
import React, { useState } from "react";
import { useBlinker } from "../../hooks";
import { SetConfig } from "../../types";
import YesNo from "../util/YesNo";
import SetupMongo from "./SetupMongo";

const Mongo = ({
  setConfig,
  next,
}: {
  setConfig: SetConfig;
  next: () => void;
}) => {
  const [setup, setSetup] = useState<boolean>();
  const [mongoURL, setMongoUrl] = useState("mongodb://127.0.0.1:27017/monitor");
  const [confirm, setConfirm] = useState(false);
  const blinker = useBlinker();

  useInput((input, key) => {
    if (setup === false) {
      if (key.return) {
        if (confirm) {
          setConfig("mongoURL", mongoURL);
          next();
        } else {
          setConfirm(true);
        }
      } else if (key.delete) {
        setMongoUrl(mongoURL.slice(0, mongoURL.length - 1));
      } else {
        setMongoUrl(mongoURL + input);
      }
    }
  });

  if (setup === undefined) {
    return (
      <YesNo
        label={
          <Text>
            Do you need to set up mongo db locally? This will begin the{" "}
            <Text color="cyan" bold>Mongo Setup Helper</Text>.
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
      <SetupMongo
        blinker={blinker}
        onFinished={(mongoURL) => {
          setConfig("mongoURL", mongoURL);
          next();
        }}
      />
    );
  } else {
    return (
      <Box flexDirection="column">
        <Box flexDirection="row">
          <Text color="green">mongo url: </Text>
          <Text>
            {mongoURL}
            {blinker && !confirm ? "|" : ""}
          </Text>
        </Box>
        <Newline />
        {confirm && (
          <Text color="green" bold>
            press enter to confirm mongo url
          </Text>
        )}
      </Box>
    );
  }
};

export default Mongo;
