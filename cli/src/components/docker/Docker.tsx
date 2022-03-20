import React, { Fragment, useEffect } from "react";
import { Newline, Text } from "ink";
import Link from "ink-link";

const Docker = () => {
  useEffect(() => {
    process.exit();
  }, []);

  // if (installDocker === undefined) {
  //   return (
  //     <YesNo
  //       label={
  //         <Text>
  //           Docker does not appear to be accessable. Would you like to{" "}
  //           <Text color="green">install docker</Text>? This will begin the{" "}
  //           <Text color="cyan" bold>
  //             Docker Install Helper
  //           </Text>
  //           . Docker is necessary to proceed.
  //         </Text>
  //       }
  //       onSelect={(res) => {
  //         setInstallDocker(res === "yes");
  //       }}
  //       vertical
  //     />
  //   );
  // } else if (installDocker) {
  //   return <InstallDocker next={next} />;
  // } else {
  //   return (
  //     <Fragment>
  //       <Text>
  //         install docker and restart the CLI to proceed. make sure that docker
  //         is accessable on the command line{" "}
  //         <Text color="green">without using sudo</Text>.
  //       </Text>
  //       <Newline />
  //     </Fragment>
  //   );
  // }
  return (
    <Fragment>
      <Text>
        docker appears appears to be inaccessable.{" "}
        <Link url="https://docs.docker.com/engine/install/">
          <Text color="blue" bold>
            install docker
          </Text>
        </Link>{" "}
        and restart the CLI to proceed. make sure that docker is accessable on
        the command line{" "}
        <Link url="https://docs.docker.com/engine/install/linux-postinstall/">
          <Text color="green" bold>
            without using sudo
          </Text>
        </Link>
        .
      </Text>
      <Newline />
    </Fragment>
  );
};

export default Docker;
