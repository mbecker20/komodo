import { Component } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../shared/Input";
import Grid from "../../../shared/layout/Grid";
import { useConfig } from "../Provider";
import Flex from "../../../shared/layout/Flex";

const OnClone: Component = () => {
  const { build, setBuild, userCanUpdate } = useConfig();
  return (
    <Grid class={combineClasses("config-item shadow")}>
      <h1>on clone</h1>
      <Flex
        alignItems="center"
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>path:</h2>
        <Input
          placeholder="relative to repo"
          value={build.on_clone?.path || ""}
          onEdit={(path) => {
            if (
              path.length === 0 &&
              (!build.on_clone ||
                !build.on_clone.command ||
                build.on_clone.command.length === 0)
            ) {
              setBuild("on_clone", undefined);
            }
            setBuild("on_clone", { path });
          }}
          disabled={!userCanUpdate()}
        />
      </Flex>
      <Flex
        alignItems="center"
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>command:</h2>
        <Input
          placeholder="command"
          value={build.on_clone?.command || ""}
          onEdit={(command) => {
            if (
              command.length === 0 &&
              (!build.on_clone ||
                !build.on_clone.path ||
                build.on_clone.path.length === 0)
            ) {
              setBuild("on_clone", undefined);
            }
            setBuild("on_clone", { command });
          }}
          disabled={!userCanUpdate()}
        />
      </Flex>
    </Grid>
  );
};

export default OnClone;
