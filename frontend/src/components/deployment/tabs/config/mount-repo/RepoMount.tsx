import { Component } from "solid-js";
import { combineClasses } from "../../../../../util/helpers";
import Input from "../../../../shared/Input";
import Flex from "../../../../shared/layout/Flex";
import Grid from "../../../../shared/layout/Grid";
import { useConfig } from "../Provider";

const RepoMount: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  return (
    <Grid class={combineClasses("config-item shadow")}>
      <h1>mount</h1>
      <Flex
        alignItems="center"
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        style={{ "flex-wrap": "wrap" }}
      >
        <Input
          placeholder="repo folder to mount"
          value={deployment.repo_mount?.local || ""}
          style={{ width: "40%" }}
          onEdit={(local) => setDeployment("repo_mount", { local })}
          disabled={!userCanUpdate()}
        />
        {" : "}
        <Input
          placeholder="container mount point"
          value={deployment.repo_mount?.container || ""}
          style={{ width: "40%" }}
          onEdit={(container) => setDeployment("repo_mount", { container })}
          disabled={!userCanUpdate()}
        />
      </Flex>
    </Grid>
  );
};

export default RepoMount;
