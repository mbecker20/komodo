import React, { ReactNode, useState } from "react";
import { Newline, render, Text, useInput, Box } from "ink";
import init from "./util/init";
import Intro from "./components/Intro";
import Docker from "./components/docker/Docker";
import Periphery from "./components/Periphery";
import Confirm from "./components/Confirm";
import { createUseConfig, createUseSequence } from "./util/state";
import { Config } from "./types";

export const useMainSequence = createUseSequence();
export const useConfig = createUseConfig<Config>({});

init().then(({ flags, dockerInstalled }) => {
  const App = () => {
    const { current, next, prev } = useMainSequence();
    const [periphery, setPeriphery] = useState<boolean | undefined>(
      flags.core ? true : flags.periphery ? false : undefined
    );
    const [installing, setInstalling] = useState(false);

    useInput((_, key) => {
      if (!installing && key.escape) prev();
    });

    const corePages: ReactNode[] = periphery === false ? [] : [];

    const peripheryPages: ReactNode[] = periphery ? [] : [];

    const pages: ReactNode[] = [
      <Intro />,
      ...(dockerInstalled ? [] : [<Docker />]),
      ...(!flags.core && !flags.periphery
        ? [<Periphery setPeriphery={setPeriphery} />]
        : []),
      ...(periphery === true ? peripheryPages : []),
      ...(periphery === false ? corePages : []),
      <Confirm
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
