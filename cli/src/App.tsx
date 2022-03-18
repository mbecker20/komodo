import React, { ReactNode, useState } from "react";
import { Box, Newline, Text, useInput } from "ink";
import Builds from "./components/Builds";
import FinalConfig from "./components/FinalConfig";
import { useConfig, useSequence } from "./hooks";
import { Config } from "./types";
import Intro from "./components/Intro";
import Setup from "./components/Setup";
import Periphery from "./components/Periphery";
import SetupMongo from "./components/SetupMongo";

const App = () => {
  const [current, next, prev] = useSequence();
  const [periphery, setPeriphery] = useState<boolean>();
  const [installing, setInstalling] = useState(false);
  const [config, setConfig] = useConfig<Config>({
    useBuilds: false,
    mongoURL: ""
  });

  const corePages: ReactNode[] = [<Builds setConfig={setConfig} next={next} />];

  const peripheryPages: ReactNode[] = [];

  const pages: ReactNode[] = [
    <Intro next={next} />,
    <Periphery setPeriphery={setPeriphery} next={next} />,
    ...(periphery === true ? peripheryPages : []),
    ...(periphery === false ? corePages : []),
    <FinalConfig
      periphery={periphery}
      config={config}
      onConfirm={() => {
        next();
        setInstalling(true);
      }}
    />,
    <Setup periphery={periphery} />,
  ];

  useInput((_, key) => {
    if (!installing && key.escape) prev();
  });
  return (
    // <Box flexDirection="column">
    //   <Newline />
    //   <Text color="blue" bold underline>
    //     Monitor CLI
    //   </Text>
    //   <Newline />
    //   {pages[current]}
    // </Box>
    <SetupMongo />
  );
};

export default App;
