import { Component } from "solid-js";
import { combineClasses } from "../../../../../util/helpers";
import Input from "../../../../shared/Input";
import Flex from "../../../../shared/layout/Flex";
import Grid from "../../../../shared/layout/Grid";
import { useConfig } from "../Provider";

export const OnClone: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
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
          value={deployment.on_clone?.path || (userCanUpdate() ? "" : "/")}
          onEdit={(path) => {
            if (
              path.length === 0 &&
              (!deployment.on_clone ||
                !deployment.on_clone.command ||
                deployment.on_clone.command.length === 0)
            ) {
              setDeployment("on_clone", undefined);
            }
            setDeployment("on_clone", { path });
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
          value={deployment.on_clone?.command || ""}
          onEdit={(command) => {
            if (
              command.length === 0 &&
              (!deployment.on_clone ||
                !deployment.on_clone.path ||
                deployment.on_clone.path.length === 0)
            ) {
              setDeployment("on_clone", undefined);
            }
            setDeployment("on_clone", { command });
          }}
          disabled={!userCanUpdate()}
        />
      </Flex>
    </Grid>
  );
};

export const OnPull: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  return (
    <Grid class={combineClasses("config-item shadow")}>
      <h1>on pull</h1>
      <Flex
        alignItems="center"
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>path:</h2>
        <Input
          placeholder="relative to repo"
          value={deployment.on_pull?.path || ""}
          onEdit={(path) => {
            if (
              path.length === 0 &&
              (!deployment.on_pull ||
                !deployment.on_pull.command ||
                deployment.on_pull.command.length === 0)
            ) {
              setDeployment("on_pull", undefined);
            }
            setDeployment("on_pull", { path });
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
          value={deployment.on_pull?.command || ""}
          onEdit={(command) => {
            if (
              command.length === 0 &&
              (!deployment.on_pull ||
                !deployment.on_pull.path ||
                deployment.on_pull.path.length === 0)
            ) {
              setDeployment("on_pull", undefined);
            }
            setDeployment("on_pull", { command });
          }}
          disabled={!userCanUpdate()}
        />
      </Flex>
    </Grid>
  );
};
