import { Component } from "solid-js";
import { useTheme } from "../../../../../state/ThemeProvider";
import { combineClasses } from "../../../../../util/helpers";
import Input from "../../../../util/Input";
import Flex from "../../../../util/layout/Flex";
import Grid from "../../../../util/layout/Grid";
import { useConfig } from "../Provider";

export const OnClone: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
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
          value={deployment.onClone?.path || (userCanUpdate() ? "" : "/")}
          onEdit={(path) => {
            if (
              path.length === 0 &&
              (!deployment.onClone ||
                !deployment.onClone.command ||
                deployment.onClone.command.length === 0)
            ) {
              setDeployment("onClone", undefined);
            }
            setDeployment("onClone", { path });
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
          value={deployment.onClone?.command || ""}
          onEdit={(command) => {
            if (
              command.length === 0 &&
              (!deployment.onClone ||
                !deployment.onClone.path ||
                deployment.onClone.path.length === 0)
            ) {
              setDeployment("onClone", undefined);
            }
            setDeployment("onClone", { command });
          }}
          disabled={!userCanUpdate()}
        />
      </Flex>
    </Grid>
  );
};

export const OnPull: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Grid class={combineClasses("config-item shadow", themeClass())}>
      <h1>on pull</h1>
      <Flex
        alignItems="center"
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>path:</h2>
        <Input
          placeholder="relative to repo"
          value={deployment.onPull?.path || ""}
          onEdit={(path) => {
            if (
              path.length === 0 &&
              (!deployment.onPull ||
                !deployment.onPull.command ||
                deployment.onPull.command.length === 0)
            ) {
              setDeployment("onPull", undefined);
            }
            setDeployment("onPull", { path });
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
          value={deployment.onPull?.command || ""}
          onEdit={(command) => {
            if (
              command.length === 0 &&
              (!deployment.onPull ||
                !deployment.onPull.path ||
                deployment.onPull.path.length === 0)
            ) {
              setDeployment("onPull", undefined);
            }
            setDeployment("onPull", { command });
          }}
          disabled={!userCanUpdate()}
        />
      </Flex>
    </Grid>
  );
};
