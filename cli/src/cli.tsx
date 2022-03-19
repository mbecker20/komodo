import React from "react";
import { render } from "ink";
import { Box } from "ink";
import { checkDockerNotInstalled } from "./helpers/docker";

const App = () => {
  return <Box></Box>;
};

export let dockerNotInstalled = true;
checkDockerNotInstalled().then((res) => {
  dockerNotInstalled = res;
  render(<App />);
});