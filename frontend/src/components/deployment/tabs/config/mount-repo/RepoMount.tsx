import { Component } from "solid-js";
import { useTheme } from "../../../../../state/ThemeProvider";
import { combineClasses } from "../../../../../util/helpers";
import Input from "../../../../util/Input";
import Flex from "../../../../util/layout/Flex";
import Grid from "../../../../util/layout/Grid";
import { useConfig } from "../Provider";

const RepoMount: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Grid class={combineClasses("config-item shadow", themeClass())}>
      <h1>mount</h1>
      <Flex
        alignItems="center"
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        style={{ "flex-wrap": "wrap" }}
      >
        <Input
          placeholder="repo folder to mount"
          value={deployment.repoMount || ""}
          style={{ width: "40%" }}
          onEdit={(value) => setDeployment("repoMount", value)}
          disabled={!userCanUpdate()}
        />
        {" : "}
        <Input
          placeholder="container mount point"
          value={deployment.containerMount || ""}
          style={{ width: "40%" }}
          onEdit={(value) => setDeployment("containerMount", value)}
          disabled={!userCanUpdate()}
        />
      </Flex>
    </Grid>
  );
};

export default RepoMount;
