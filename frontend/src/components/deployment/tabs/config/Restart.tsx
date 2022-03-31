import { Component } from "solid-js";
import Flex from "../../../util/layout/Flex";
import Selector from "../../../util/menu/Selector";
import { useConfig } from "./Provider";

const RESTART_MODES = [
  "don't restart",
  "unless stopped",
  "on failure",
  "always",
];

const Restart: Component<{}> = (p) => {
  const { deployment, setDeployment } = useConfig();
  return (
    <Flex
      class="config-item shadow"
      justifyContent="space-between"
    >
      <h1>restart</h1>
      <Selector
        targetClass="blue"
        items={RESTART_MODES}
        selected={
          (deployment.restart === "no"
            ? "don't restart"
            : deployment.restart?.replace("-", " ")) || "don't restart"
        }
        onSelect={(restart) =>
          setDeployment(
            "restart",
            restart === "don't restart" ? "no" : restart.replace(" ", "-")
          )
        }
        position="bottom right"
      />
    </Flex>
  );
};

export default Restart;
