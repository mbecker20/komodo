import { Component } from "solid-js";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import { useConfig } from "../Provider";

const CliBuild: Component<{}> = (p) => {
  const { build, setBuild, userCanUpdate } = useConfig();
  return (
    <Grid class="config-item shadow">
      <h1>cli build</h1>
      {/* <div style={{ opacity: 0.7 }}>build with a custom command</div> */}
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
      >
        <h2>build path: </h2>
        <Input
          placeholder="from root of repo"
          value={build.cliBuild?.path || (userCanUpdate() ? "" : "/")}
          onEdit={(path) => setBuild("cliBuild", { path })}
          disabled={!userCanUpdate()}
        />
      </Flex>
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
      >
        <h2>command: </h2>
        <Input
          placeholder="ie. yarn build"
          value={build.cliBuild?.command || ""}
          onEdit={(command) => setBuild("cliBuild", { command })}
          disabled={!userCanUpdate()}
        />
      </Flex>
    </Grid>
  );
};

export default CliBuild;
