import { Component } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../util/Input";
import Grid from "../../../util/layout/Grid";
import { useConfig } from "../Provider";
import s from "../../build.module.css";
import Flex from "../../../util/layout/Flex";
import { useTheme } from "../../../../state/ThemeProvider";

const OnClone: Component = () => {
  const { build, setBuild, userCanUpdate } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Grid class={combineClasses("config-item shadow", themeClass())}>
      <h1>on clone</h1>
      <Flex
        alignItems="center"
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>path:</h2>
        <Input
          placeholder="relative to repo"
          value={build.onClone?.path || ""}
          onEdit={(path) => {
            if (
              path.length === 0 &&
              (!build.onClone ||
                !build.onClone.command ||
                build.onClone.command.length === 0)
            ) {
              setBuild("onClone", undefined);
            }
            setBuild("onClone", { path });
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
          value={build.onClone?.command || ""}
          onEdit={(command) => {
            if (
              command.length === 0 &&
              (!build.onClone ||
                !build.onClone.path ||
                build.onClone.path.length === 0)
            ) {
              setBuild("onClone", undefined);
            }
            setBuild("onClone", { command });
          }}
          disabled={!userCanUpdate()}
        />
      </Flex>
    </Grid>
  );
};

export default OnClone;
