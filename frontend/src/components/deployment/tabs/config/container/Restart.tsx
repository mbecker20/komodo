import { Component } from "solid-js";
import { RestartMode } from "../../../../../types";
import { combineClasses } from "../../../../../util/helpers";
import Flex from "../../../../shared/layout/Flex";
import Selector from "../../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const RESTART_MODES = [
  "don't restart",
  "unless stopped",
  "on failure",
  "always",
];

const Restart: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  return (
    <Flex
      class={combineClasses("config-item shadow")}
      justifyContent="space-between"
      alignItems="center"
    >
      <h1>restart</h1>
      <Selector
        targetClass="blue"
        items={RESTART_MODES}
        selected={
          (deployment.docker_run_args.restart === "no"
            ? "don't restart"
            : deployment.docker_run_args.restart?.replace("-", " ")) ||
          "don't restart"
        }
        onSelect={(restart) =>
          setDeployment("docker_run_args", {
            restart:
              restart === "don't restart"
                ? RestartMode.NoRestart
                : (restart.replace(" ", "-") as RestartMode),
          })
        }
        position="bottom right"
        disabled={!userCanUpdate()}
      />
    </Flex>
  );
};

export default Restart;
