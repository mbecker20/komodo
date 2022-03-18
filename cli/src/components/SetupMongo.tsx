import { CommandLogError } from "@monitor/types";
import { Box, Newline, Text, useInput } from "ink";
import React, { useState } from "react";
import { startMongo } from "../helpers/mongo";
import { useBlinker } from "../hooks";
import Selector from "./util/Selector";

const RESTART_MODES = ["no", "on-failure", "always", "unless-stopped"];

const SetupMongo = ({}: {}) => {
  const [stage, setStage] = useState<
    "name" | "port" | "volume" | "restart" | "confirm"
  >("name"); // 0: name, 1: port, 2: volume, 3: restart
  const [name, setName] = useState("mongo-db");
  const [port, setPort] = useState<string>();
  const [volume, setVolume] = useState<string | false>();
  const [restart, setRestart] = useState<string>();
  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<CommandLogError>();
  const blinker = useBlinker();
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
            setRestart("");
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
          startMongo(name, Number(port!), volume!, restart!).then((res) =>
            setResult(res)
          );
          break;

        default:
          break;
      }
    } else if (key.escape) {
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
          setStage("restart");
          setRestart("");
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
      <Text color="cyan">start mongo</Text>
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
        <Box>
          <Text color="green">use volume? </Text>
          <Selector
            items={["yes", "no"]}
            onSelect={(use) => {
              if (use === "yes") {
                setVolume("~/mongo");
              } else {
                setVolume(false);
                setRestart(""); /*  */
                setStage("restart");
              }
            }}
          />
        </Box>
      )}
      {(volume || volume === false) && (
        <Text color="green">
          volume:{" "}
          <Text color="white">
            {volume || "false"}
            {stage === "volume" && blinker ? "|" : ""}
          </Text>
        </Text>
      )}
      {restart !== undefined && (
        <Box>
          <Text color="green">restart: </Text>
          {restart.length === 0 ? (
            <Selector
              items={RESTART_MODES}
              onSelect={(mode) => {
                setRestart(mode);
                setStage("confirm");
              }}
            />
          ) : (
            <Text>{restart}</Text>
          )}
        </Box>
      )}
      {stage === "confirm" && (
        <Box flexDirection="column">
          <Newline />
          <Text color="green">press enter to start mongo...</Text>
          {running ? <Text color="cyan">running...</Text> : undefined}
        </Box>
      )}
      {result && (
        <Box flexDirection="column">
          <Newline />
          <Text>command: {result.command}</Text>
          {result.log.stdout && (
            <Box flexDirection="column">
              <Newline />
              <Text>stdout: {result.log.stdout}</Text>
            </Box>
          )}
          {result.log.stderr ? (
            <Box flexDirection="column">
              <Newline />
              <Text>stderr: {result.log.stderr}</Text>
            </Box>
          ) : undefined}
        </Box>
      )}
    </Box>
  );
};

export default SetupMongo;
