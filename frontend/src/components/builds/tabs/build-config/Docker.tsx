import { Component } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import s from "../../build.module.css";
import { useConfig } from "../Provider";

const Docker: Component<{}> = (p) => {
  const { build, setBuild } = useConfig();
  return (
    <Grid class={combineClasses(s.ConfigItem, "shadow")}>
      <h1>docker build</h1> {/* checkbox here? */}
      <Flex justifyContent="space-between" alignItems="center">
        <div>build path</div>
        <Input
          placeholder="from root of repo"
          value={build.dockerBuildArgs?.buildPath || ""}
          onEdit={(buildPath) => setBuild("dockerBuildArgs", { buildPath })}
        />
      </Flex>
      <Flex justifyContent="space-between" alignItems="center">
        <div>dockerfile path</div>
        <Input
          placeholder="from root of build path"
          value={build.dockerBuildArgs?.dockerfilePath || ""}
          onEdit={(dockerfilePath) => setBuild("dockerBuildArgs", { dockerfilePath })}
        />
      </Flex>
    </Grid>
  );
};

export default Docker;
