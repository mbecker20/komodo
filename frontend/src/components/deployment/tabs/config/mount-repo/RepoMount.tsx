import { Component } from "solid-js";
import Input from "../../../../util/Input";
import Flex from "../../../../util/layout/Flex";
import Grid from "../../../../util/layout/Grid";
import { useConfig } from "../Provider";

const RepoMount: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  return (
    <Grid class="config-item shadow">
      <h1>mount</h1>
      <Flex
        alignItems="center"
        justifyContent={userCanUpdate() ? "space-between" : undefined}
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
