import React, { ReactNode, useState } from "react";
import { Newline, render, Text, Box } from "ink";
import init from "./util/init";
import Intro from "./components/Intro";
import Docker from "./components/docker/Docker";
import IsPeriphery from "./components/IsPeriphery";
import Confirm from "./components/Confirm";
import { createUseConfig, createUseSequence } from "./util/state";
import { Config } from "./types";
import Mongo from "./components/deployment-config/Mongo";
// import Registry from "./components/deployment-config/Registry";
import CoreOrPeriphery from "./components/core-or-periphery/CoreOrPeriphery";
import { bound } from "./util/helpers/general";
import Setup from "./components/Setup";
import Restart from "./components/Restart";

type Page = {
  title: string;
  view: ReactNode;
};

export const useMainSequence = createUseSequence();
export const useConfig = createUseConfig<Config>({});

init().then(({ flags, dockerInstalled }) => {
  const App = () => {
    const { current } = useMainSequence();
    const [periphery, setPeriphery] = useState<boolean | undefined>(
      flags.core ? false : flags.periphery ? true : undefined
    );

    if (flags.restart || flags.restartDefault) {
      return (
        <Box flexDirection="column">
          <Newline />
          <Box>
            <Text color="blue" bold underline>
              Monitor CLI{" "}
            </Text>
            <Box marginLeft={2}>
              <Text color="gray">restart {`(1 of 1)`}</Text>
            </Box>
          </Box>
          <Newline />
          <Restart
            useDefaults={flags.restartDefault ? true : false}
            defaultName={flags.name}
            defaultMongoUrl={flags.mongoUrl}
          />
        </Box>
      );
    }

    const corePages: Page[] = [
      {
        title: "mongo config",
        view: <Mongo />,
      },
      // {
      //   title: "registry config",
      //   view: <Registry />,
      // },
      {
        title: "monitor core config",
        view: <CoreOrPeriphery type="core" />,
      },
    ];

    const peripheryPages: Page[] = [
      {
        title: "periphery config",
        view: <CoreOrPeriphery type="periphery" />,
      },
    ];

    const pages = [
      {
        title: "intro",
        view: <Intro />,
      },
      dockerInstalled
        ? false
        : {
            title: "docker intro",
            view: <Docker />,
          },
      {
        title: "restart",
        view: <Restart />,
      },
      !flags.core && !flags.periphery
        ? {
            title: "core or periphery",
            view: <IsPeriphery setPeriphery={setPeriphery} />,
          }
        : false,
      periphery === true && peripheryPages,
      periphery === false && corePages,
      {
        title: "confirm config",
        view: <Confirm />,
      },
      {
        title: "setup",
        view: <Setup />,
      },
    ]
      .filter((val) => (val ? true : false))
      .flat();

    const { title, view } = pages[bound(current, 0, pages.length - 1)] as Page;

    return (
      <Box flexDirection="column">
        <Newline />
        <Box>
          <Text color="blue" bold underline>
            Monitor CLI{" "}
          </Text>
          <Box marginLeft={2}>
            <Text color="gray">
              {title} {`(${current + 1} of ${pages.length})`}
            </Text>
          </Box>
        </Box>
        <Newline />
        {view}
      </Box>
    );
  };

  render(<App />);
});
