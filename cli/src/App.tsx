import React, { ReactNode, useState } from "react";
import { Box, Newline, Text, useInput } from "ink";
import Builds from "./components/Builds";
import FinalConfig from "./components/FinalConfig";
import { useConfig, useSequence } from "./hooks";
import { Config } from "./types";
import Intro from "./components/Intro";
import Setup from "./components/Setup";
import Periphery from "./components/Periphery";
import Mongo from "./components/mongo/Mongo";
import { dockerNotInstalled } from "./monitor-cli";
import Docker from "./components/docker/Docker";
import Registry from "./components/registry/Registry";

const App = () => {
  const [current, next, prev] = useSequence();
  const [periphery, setPeriphery] = useState<boolean>();
  const [installing, setInstalling] = useState(false);
  const [config, setConfig] = useConfig<Config>({
    useBuilds: false,
    mongoURL: "",
    registryURL: "",
  });

  const corePages: ReactNode[] = [
    <Mongo setConfig={setConfig} next={next} />,
    <Registry setConfig={setConfig} next={next} />,
    <Builds setConfig={setConfig} next={next} />,
  ];

  const peripheryPages: ReactNode[] = [];

  const pages: ReactNode[] = [
    <Intro next={next} />,
    ...(dockerNotInstalled ? [<Docker next={next} />] : []),
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
    <Box flexDirection="column">
      <Newline />
      <Text color="blue" bold underline>
        Monitor CLI
      </Text>
      <Newline />
      {pages[current]}
    </Box>
  );
};

export default App;
