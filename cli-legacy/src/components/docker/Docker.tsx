import React, { useEffect, useState } from "react";
import { Text } from "ink";
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
            Docker does not appear to be installed. Would you like to install
            Docker? This will begin the{" "}
            <Text color="cyan" bold>
              Docker Install Helper
            </Text>{" "}
            and is necessary to proceed.
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
    return <Text>Install docker and restart the CLI to proceed.</Text>;
  }
};

export default Docker;
