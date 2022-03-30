import { Component } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Grid from "../../../util/layout/Grid";
import { useConfig } from "./Provider";
import s from "../../build.module.css";
import Flex from "../../../util/layout/Flex";
import Input from "../../../util/Input";

const Git: Component<{}> = (p) => {
  const { build, setBuild } = useConfig();
  return (
    <Grid class={combineClasses(s.ConfigItem, "shadow")}>
      <h1>github config</h1>
      <Flex justifyContent="space-between" alignItems="center">
        <div>repo</div>
        <Input
          placeholder="ie. solidjs/solid"
          value={build.repo}
          onEdit={(value) => setBuild("repo", value)}
        />
      </Flex>
      <Flex justifyContent="space-between" alignItems="center">
        <div>branch</div>
        <Input
          placeholder="defaults to main"
          value={build.branch}
          onEdit={(value) => setBuild("branch", value)}
        />
      </Flex>
      <Flex justifyContent="space-between" alignItems="center">
        <div>access token</div>
        <Input
          placeholder="paste token"
          value={build.accessToken}
          onEdit={(value) => setBuild("accessToken", value)}
        />
      </Flex>
    </Grid>
  );
};

export default Git;