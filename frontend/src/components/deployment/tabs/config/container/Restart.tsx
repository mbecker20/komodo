import { Component } from "solid-js";
import { useTheme } from "../../../../../state/ThemeProvider";
import { combineClasses } from "../../../../../util/helpers";
import Flex from "../../../../util/layout/Flex";
import Selector from "../../../../util/menu/Selector";
import { useConfig } from "../Provider";

const RESTART_MODES = [
  "don't restart",
  "unless stopped",
  "on failure",
  "always",
];

const Restart: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Flex
      class={combineClasses("config-item shadow", themeClass())}
      justifyContent="space-between"
      alignItems="center"
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
        disabled={!userCanUpdate()}
      />
    </Flex>
  );
};

export default Restart;
