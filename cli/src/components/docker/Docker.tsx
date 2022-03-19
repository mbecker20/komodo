import React, { Fragment, useEffect, useState } from "react";
import { Newline, Text } from "ink";
import { Next } from "../../types";
import YesNo from "../util/YesNo";
import InstallDocker from "./InstallDocker";

const Docker = ({ next }: { next: Next }) => {
  const [installDocker, setInstallDocker] = useState<boolean>();
  useEffect(() => {
    if (installDocker === false) {
      process.exit();
    }
  }, [installDocker]);
  if (installDocker === undefined) {
    return (
      <YesNo
        label={
          <Text>
            Docker does not appear to be accessable. Would you like to{" "}
            <Text color="green">install docker</Text>? This will begin the{" "}
            <Text color="cyan" bold>
              Docker Install Helper
            </Text>
            . Docker is necessary to proceed.
          </Text>
        }
        onSelect={(res) => {
          setInstallDocker(res === "yes");
        }}
        direction="vertical"
      />
    );
  } else if (installDocker) {
    return <InstallDocker next={next} />;
  } else {
    return (
      <Fragment>
        <Text>
          install docker and restart the CLI to proceed. make sure that docker
          is accessable on the command line{" "}
          <Text color="green">without using sudo</Text>.
        </Text>
        <Newline />
      </Fragment>
    );
  }
};

export default Docker;
