import React, { Fragment, useState } from "react";
import { Box, Newline, Text, useInput } from "ink";
import { Next, SetConfig } from "../../types";
import YesNo from "../util/YesNo";
import { installDockerUbuntu, InstallLog } from "../../helpers/docker";

const InstallDocker = ({ next }: { next: Next }) => {
  const [stage, setStage] = useState<
    | "userGroup"
    | "sysCtlEnable"
    | "confirm"
    | "install"
    | "installing"
    | "finish"
    | "error"
  >("userGroup");
  const [addToUG, setAddToUG] = useState<"yes" | "no">();
  const [sysCtlEnable, setSysCtlEnable] = useState<"yes" | "no">();
  const [logs, setLogs] = useState<InstallLog[]>([]);
  useInput(async (_, key) => {
    if (key.return) {
      switch (stage) {
        case "confirm":
          const log = await installDockerUbuntu(
            (log) => setLogs((logs) => [...logs, log]),
            addToUG === "yes",
            sysCtlEnable === "yes"
          );
          if (log) {
            // there was some error
						setLogs((logs) => [...logs, log])
            setStage("error");
          } else {
            setStage("finish");
          }
          break;

        case "finish":
					next();
          break;

				case "error":
					setAddToUG(undefined);
					setSysCtlEnable(undefined);
					setStage("userGroup");
					break;

        default:
          break;
      }
    } else if (key.leftArrow) {
			switch (stage) {
				case "sysCtlEnable":
					setAddToUG(undefined);
					setStage("userGroup");
					break;

				case "confirm":
					setSysCtlEnable(undefined);
					setStage("sysCtlEnable");
					break;
			
				default:
					break;
			}
		}
  });
  return (
    <Box flexDirection="column">
      <Text color="cyan" bold>
        Docker Install Helper
      </Text>
      <Newline />
      {addToUG === undefined && (
        <YesNo
          label="add to user group? this will allow for the use of docker without sudo."
          labelColor="white"
          onSelect={(res) => {
            setAddToUG(res);
            setStage("sysCtlEnable");
          }}
          direction="vertical"
        />
      )}
      {addToUG !== undefined && (
        <Text color="green">
          add to user group: <Text color="white">{addToUG}</Text>
        </Text>
      )}
      {stage === "sysCtlEnable" && sysCtlEnable === undefined && (
        <YesNo
          label="start docker on system start (boot)?"
          labelColor="white"
          onSelect={(res) => {
            setSysCtlEnable(res);
            setStage("confirm");
          }}
          direction="vertical"
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
            press <Text color="green">enter</Text> to install docker
          </Text>
        </Fragment>
      )}
      {(stage === "installing" || stage === "finish") && (
        <Fragment>
          {stage === "installing" && <Text color="cyan">installing...</Text>}
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
      {stage === "finish" && (
        <Text>
          Docker has finished installing. Press <Text color="green">enter</Text>{" "}
          to continue.
        </Text>
      )}
      {stage === "error" && (
        <Text>
          There was an error during install. Press{" "}
          <Text color="green">enter</Text> to try again.
        </Text>
      )}
    </Box>
  );
};

export default InstallDocker;
