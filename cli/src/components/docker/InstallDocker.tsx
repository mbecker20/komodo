import React, { Fragment, useState } from "react";
import { Box, Newline, Text } from "ink";
import YesNo from "../util/YesNo";
import { installDockerUbuntu, InstallLog } from "../../util/helpers/docker";
import { useEnter, useKey } from "../../util/hooks";
import Spinner from "ink-spinner";

const InstallDocker = ({ next }: { next: () => void }) => {
  const [stage, setStage] = useState<
    "sysCtlEnable" | "confirm" | "install" | "installing" | "finish" | "error"
  >("sysCtlEnable");
  const [sysCtlEnable, setSysCtlEnable] = useState<"yes" | "no">();
  const [logs, setLogs] = useState<InstallLog[]>([]);
  useEnter(async () => {
    switch (stage) {
      case "confirm":
        setStage("installing");
        const log = await installDockerUbuntu(
          (log) => setLogs((logs) => [...logs, log]),
          sysCtlEnable === "yes"
        );
        if (log) {
          // there was some error
          setLogs((logs) => [...logs, log]);
          setStage("error");
        } else {
          setStage("finish");
        }
        break;

      case "finish":
        next();
        break;

      case "error":
        setSysCtlEnable(undefined);
        setStage("sysCtlEnable");
        break;
    }
  });
  useKey("leftArrow", () => {
    switch (stage) {
      case "confirm":
        setSysCtlEnable(undefined);
        setStage("sysCtlEnable");
        break;
    }
  });
  return (
    <Box flexDirection="column">
      <Text color="cyan" bold>
        Docker Install Helper
      </Text>
      <Newline />
      {stage === "sysCtlEnable" && sysCtlEnable === undefined && (
        <YesNo
          label="start docker on system start (boot)?"
          labelColor="white"
          onSelect={(res) => {
            setSysCtlEnable(res);
            setStage("confirm");
          }}
          vertical
        />
      )}
      {sysCtlEnable !== undefined && (
        <Text color="green">
          start on boot: <Text color="white">{sysCtlEnable}</Text>
        </Text>
      )}
      {stage === "confirm" && (
        <Fragment>
          <Newline />
          <Text>
            press <Text color="green">enter</Text> to install docker. you may
            have to provide your password.
          </Text>
        </Fragment>
      )}
      {(stage === "installing" || stage === "finish") && (
        <Fragment>
          {stage === "installing" && (
            <Text>
              <Text color="green">
                <Spinner type="dots" />
              </Text>
              installing...
            </Text>
          )}
          <Newline />
          {logs.map(({ stage, log }) => {
            <Fragment>
              <Text color="cyan" bold>
                {stage}
              </Text>
              <Text color="green">
                command: <Text color="white">{log.command}</Text>
              </Text>
              {log.log.stdout ? (
                <Text color="green">
                  stdout: <Text color="white">{log.log.stdout}</Text>
                </Text>
              ) : undefined}
              {log.log.stderr ? (
                <Text color="red">
                  stderr: <Text color="white">{log.log.stderr}</Text>
                </Text>
              ) : undefined}
              <Newline />
            </Fragment>;
          })}
        </Fragment>
      )}
      <Newline />
      {stage === "finish" && (
        <Text>
          docker has finished installing. press <Text color="green">enter</Text>{" "}
          to continue.
        </Text>
      )}
      {stage === "error" && (
        <Text>
          there was an error during install. press{" "}
          <Text color="green">enter</Text> to try again.
        </Text>
      )}
    </Box>
  );
};

export default InstallDocker;
