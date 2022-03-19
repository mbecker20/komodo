import React, { ReactNode, useEffect, useState } from "react";
import { Newline, render, Text, useInput, Box } from "ink";
import { isDockerInstalled } from "./helpers/docker";
import { useConfig, useSequence } from "./hooks";
import { Config } from "./types";
import Intro from "./components/Intro";
import Docker from "./components/docker/Docker";
import Periphery from "./components/Periphery";
import Confirm from "./components/Confirm";
import getFlags from "./flags";

getFlags().then((flags) => {
  const App = () => {
    const [current, next, prev] = useSequence();
    const [dockerInstalled, setDockerInstalled] = useState<boolean>();
    const [periphery, setPeriphery] = useState<boolean | undefined>(
      flags.core ? true : flags.periphery ? false : undefined
    );
    const [installing, setInstalling] = useState(false);
    const [config, setConfig] = useConfig<Config>({});

    useEffect(() => {
      isDockerInstalled().then((res) => setDockerInstalled(res));
    }, []);

    useInput((_, key) => {
      if (!installing && key.escape) prev();
    });

    if (dockerInstalled === undefined) return null;

    const corePages: ReactNode[] = periphery === false ? [] : [];

    const peripheryPages: ReactNode[] = periphery ? [] : [];

    const pages: ReactNode[] = [
      <Intro next={next} />,
      ...(dockerInstalled ? [] : [<Docker next={next} />]),
      ...(!flags.core && !flags.periphery
        ? [<Periphery setPeriphery={setPeriphery} next={next} />]
        : []),
      ...(periphery === true ? peripheryPages : []),
      ...(periphery === false ? corePages : []),
      <Confirm
        config={config}
        next={() => {
          setInstalling(true);
          next();
        }}
      />,
    ];
    return (
      <Box flexDirection="column">
        <Newline />
        <Text color="blue" bold underline>
          monitor CLI
        </Text>
        <Newline />
        {pages[current]}
      </Box>
    );
  };

  render(<App />);
});
