import React, { useState } from "react";
import { Text } from "ink";
import { Next } from "../../types";
import YesNo from "../util/YesNo";
import InstallDocker from "./InstallDocker";

const Docker = ({ next }: { next: Next }) => {
  const [installDocker, setInstallDocker] = useState<boolean>();
  if (installDocker === undefined) {
    return (
      <YesNo
        label={
          <Text>
            Would you like to install Docker? This will begin the{" "}
            <Text color="cyan" bold>
              Docker Install Helper
            </Text>
          </Text>
        }
        onSelect={(res) => {
					if (res === "yes") {
						setInstallDocker(true);
					} else {
						next();
					}
				}}
        direction="vertical"
      />
    );
  } else if (installDocker) {
		return <InstallDocker next={next} />
	} else {
		return null;
	}
};

export default Docker;
