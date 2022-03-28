import { Component } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Flex from "../../../util/layout/Flex";
import Selector from "../../../util/menu/Selector";
import s from "../../deployment.module.css";
import { useConfig } from "./Provider";

const RESTART_MODES = [
  "always",
  "on failure",
  "unless stopped",
  "don't restart",
];

const Restart: Component<{}> = (p) => {
  const { deployment, setDeployment } = useConfig();
  return (
    <Flex
      class={combineClasses(s.ConfigItem, "shadow")}
      justifyContent="space-between"
    >
      <h1>restart</h1>
      <Selector
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
      {/* <Input
        value={deployment.network}
        placeholder="network"
        onConfirm={(value) => setDeployment("network", value)}
      /> */}
    </Flex>
  );
};

export default Restart;
