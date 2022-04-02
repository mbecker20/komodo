import { Component } from "solid-js";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import { useConfig } from "./Provider";

export const OnClone: Component<{}> = (p) => {
  const { deployment, setDeployment } = useConfig();
  return (
    <Grid class="config-item shadow">
      <h1>on clone</h1>
      <Flex alignItems="center" justifyContent="space-between">
        path:
        <Input
          placeholder="relative to repo"
          value={deployment.onClone?.path || ""}
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
        />
      </Flex>
      <Flex alignItems="center" justifyContent="space-between">
        command:
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
        />
      </Flex>
    </Grid>
  );
};

export const OnPull: Component<{}> = (p) => {
  const { deployment, setDeployment } = useConfig();
  return (
    <Grid class="config-item shadow">
      <h1>on pull</h1>
      <Flex alignItems="center" justifyContent="space-between">
        path:
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
        />
      </Flex>
      <Flex alignItems="center" justifyContent="space-between">
        command:
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
        />
      </Flex>
    </Grid>
  );
};
