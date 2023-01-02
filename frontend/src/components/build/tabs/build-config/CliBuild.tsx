import { Component } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../shared/Input";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import { useConfig } from "../Provider";

const CliBuild: Component<{}> = (p) => {
  const { build, setBuild, userCanUpdate } = useConfig();
  return (
    <Grid class={combineClasses("config-item shadow")}>
      <h1>pre build</h1>
      {/* <div style={{ opacity: 0.7 }}>build with a custom command</div> */}
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>build path: </h2>
        <Input
          placeholder="from root of repo"
          value={build.pre_build?.path || (userCanUpdate() ? "" : "/")}
          onEdit={(path) => setBuild("pre_build", { path })}
          disabled={!userCanUpdate()}
        />
      </Flex>
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>command: </h2>
        <Input
          placeholder="ie. yarn build"
          value={build.pre_build?.command || ""}
          onEdit={(command) => setBuild("pre_build", { command })}
          disabled={!userCanUpdate()}
        />
      </Flex>
    </Grid>
  );
};

export default CliBuild;
