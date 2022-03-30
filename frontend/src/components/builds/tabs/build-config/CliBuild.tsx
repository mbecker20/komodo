import { Component } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import s from "../../build.module.css";
import { useConfig } from "../Provider";

const CliBuild: Component<{}> = (p) => {
  const { build, setBuild } = useConfig();
  return (
    <Grid class={combineClasses(s.ConfigItem, "shadow")}>
      <h1>cli build</h1>
      <div>build with a custom command</div>
      <Flex justifyContent="space-between" alignItems="center">
        <div>build path</div>
        <Input
          placeholder="from root of repo"
          value={build.cliBuild?.path || ""}
          onEdit={(path) => setBuild("cliBuild", { path })}
        />
      </Flex>
      <Flex justifyContent="space-between" alignItems="center">
        <div>command</div>
        <Input
          placeholder="ie. yarn build"
          value={build.cliBuild?.command || ""}
          onEdit={(command) => setBuild("cliBuild", { command })}
        />
      </Flex>
    </Grid>
  );
};

export default CliBuild;
