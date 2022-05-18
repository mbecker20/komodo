import { Component } from "solid-js";
import { useTheme } from "../../../../state/ThemeProvider";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import { useConfig } from "./Provider";

const Info: Component<{}> = (p) => {
  const { server, setServer } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Grid class={combineClasses("config-item shadow", themeClass())}>
      <h1>info</h1>
      <Flex justifyContent="space-between" alignItems="center">
        <div>region</div>
        <Input
          value={server.region}
          onEdit={(value) => setServer("region", value)}
          style={{ width: "12rem" }}
        />
      </Flex>
      <Flex justifyContent="space-between" alignItems="center">
        <div>instance id</div>
        <Input
          value={server.instanceID}
          onEdit={(value) => setServer("instanceID", value)}
          style={{ width: "12rem" }}
        />
      </Flex>
    </Grid>
  );
};

export default Info;
