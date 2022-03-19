import { CommandLogError } from "@monitor/types";
import { Box, Newline, Text, useInput } from "ink";
import React, { Fragment, useState } from "react";
import { startMongo } from "../../helpers/deploy/mongo";
import LabelledSelector from "../util/LabelledSelector";
import YesNo from "../util/YesNo";

const RESTART_MODES = ["no", "on-failure", "always", "unless-stopped"];

const SetupMongo = ({
  onFinished,
  blinker,
}: {
  onFinished: (mongoURL: string) => void;
  blinker: boolean;
}) => {
  const [stage, setStage] = useState<
    "name" | "port" | "volume" | "restart" | "confirm" | "finish"
  >("name"); // 0: name, 1: port, 2: volume, 3: restart
  const [name, setName] = useState("mongo-db");
  const [port, setPort] = useState<string>();
  const [volume, setVolume] = useState<string | false>();
  const [restart, setRestart] = useState<string>();
  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<CommandLogError>();
  useInput((input, key) => {
    if (key.return) {
      switch (stage) {
        case "name":
          setStage("port");
          setPort("27017");
          break;

        case "port":
          if (!isNaN(Number(port))) {
            setStage("volume");
          }
          break;

        case "volume":
          if (volume) {
            setStage("restart");
          }
          break;

        case "restart":
          if (restart) {
            setStage("confirm");
          }
          break;

        case "confirm":
          setRunning(true);
          startMongo(name, Number(port!), volume!, restart!).then((res) => {
            setResult(res);
            setStage("finish");
          });
          break;

        case "finish":
          if (!result?.isError) {
            onFinished(`mongodb://127.0.0.1:${port}/monitor`);
          }
          break;

        default:
          break;
      }
    } else if (key.leftArrow) {
      switch (stage) {
        case "port":
          setPort(undefined);
          setStage("name");
          break;

        case "volume":
          setVolume(undefined);
          setStage("port");
          break;

        case "restart":
          setRestart(undefined);
          if (volume === false) {
            setVolume(undefined);
          }
          setStage("volume");
          break;

        case "confirm":
          setRestart(undefined);
          setStage("restart");
          break;

        default:
          break;
      }
    } else if (key.delete) {
      switch (stage) {
        case "name":
          setName(name.slice(0, Math.max(0, name.length - 1)));
          break;

        case "port":
          if (port) setPort(port.slice(0, Math.max(0, port.length - 1)));
          break;

        case "volume":
          if (volume)
            setVolume(volume.slice(0, Math.max(0, volume.length - 1)));
          break;

        default:
          break;
      }
    } else {
      switch (stage) {
        case "name":
          setName(name + input);
          break;

        case "port":
          const newPort = Number(port + input);
          if (!isNaN(newPort)) {
            setPort(newPort.toString());
          }
          break;

        case "volume":
          if (volume) setVolume(volume + input);
          break;

        default:
          break;
      }
    }
  });
  return (
    <Box flexDirection="column">
      <Text color="cyan" bold>
        Mongo Setup Helper
      </Text>
      <Newline />
      <Text>
        press your keyboard back arrow ({"<-"}) to go back to the previous field
      </Text>
      <Newline />
      <Text color="green">
        container name:{" "}
        <Text color="white">
          {name}
          {stage === "name" && blinker ? "|" : ""}
        </Text>
      </Text>
      {port && (
        <Text color="green">
          port:{" "}
          <Text color="white">
            {port}
            {stage === "port" && blinker ? "|" : ""}
          </Text>
        </Text>
      )}
      {stage === "volume" && volume === undefined && (
        <LabelledSelector
          label="mount data on local filesystem:"
          items={["yes", "no"]}
          onSelect={(use) => {
            if (use === "yes") {
              setVolume("~/mongo");
            } else {
              setVolume(false);
              setStage("restart");
            }
          }}
        />
      )}
      {(volume || volume === false) && (
        <Text color="green">
          {volume ? "mount folder: " : "mount: "}
          <Text color="white">
            {volume || "no"}
            {stage === "volume" && blinker ? "|" : ""}
          </Text>
        </Text>
      )}
      {stage === "restart" && restart === undefined && (
        <LabelledSelector
          label="restart:"
          items={RESTART_MODES}
          onSelect={(mode) => {
            setRestart(mode);
            setStage("confirm");
          }}
        />
      )}
      {restart !== undefined && (
        <Text color="green">
          restart: <Text color="white">{restart}</Text>
        </Text>
      )}
      {/* {stage === "confirm" && (
        <Box flexDirection="column">
          <Newline />
          {running ? (
            <Text color="cyan">running...</Text>
          ) : (
            <Text color="green">press enter to start mongo...</Text>
          )}
        </Box>
      )} */}
      {stage === "confirm" && (
        <Fragment>
          <Newline />
          {running ? (
            <Text color="cyan">running...</Text>
          ) : (
            <Text>
              press <Text color="green">enter</Text> to start mongo...
            </Text>
          )}
        </Fragment>
      )}
      {result && (
        <Fragment>
          <Newline />
          <Text>command: {result.command}</Text>
          {result.log.stdout ? (
            <Fragment>
              <Newline />
              <Text>stdout: {result.log.stdout}</Text>
            </Fragment>
          ) : undefined}
          {result.log.stderr ? (
            <Fragment>
              <Newline />
              <Text>stderr: {result.log.stderr}</Text>
            </Fragment>
          ) : undefined}
        </Fragment>
      )}
      {stage === "finish" && result && !result.isError && (
        <Text>
          Success! Press{" "}
          <Text color="green" bold>
            enter
          </Text>{" "}
          to continue
        </Text>
      )}
      {stage === "finish" && result && result.isError && (
        <Fragment>
          <Newline />
          <YesNo
            label="looks like that failed, would you like to retry?"
            labelColor="white"
            onYes={() => {
              setPort(undefined);
              setVolume(undefined);
              setRestart(undefined);
              setResult(undefined);
              setStage("name");
            }}
            onNo={() => {
              onFinished(`mongodb://127.0.0.1:${port}/monitor`);
            }}
            direction="vertical"
          />
        </Fragment>
      )}
    </Box>
  );
};

export default SetupMongo;
