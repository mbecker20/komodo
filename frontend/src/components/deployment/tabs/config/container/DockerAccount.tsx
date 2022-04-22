import { Component, Show } from "solid-js";
import { useAppState } from "../../../../../state/StateProvider";
import { useTheme } from "../../../../../state/ThemeProvider";
import { combineClasses } from "../../../../../util/helpers";
import Flex from "../../../../util/layout/Flex";
import Selector from "../../../../util/menu/Selector";
import { useConfig } from "../Provider";

const DockerAccount: Component<{}> = (p) => {
	const { dockerAccounts } = useAppState();
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Show when={dockerAccounts() && dockerAccounts()!.length > 0}>
      <Flex
        class={combineClasses("config-item shadow", themeClass())}
        justifyContent="space-between"
        alignItems="center"
        style={{ "flex-wrap": "wrap" }}
      >
        <h1>docker account</h1>
        <Selector
          targetClass="blue"
          items={["none", ...dockerAccounts()!]}
          selected={deployment.dockerAccount || "none"}
          onSelect={(account) =>
            setDeployment(
              "dockerAccount",
              account === "none" ? undefined : account
            )
          }
          position="bottom right"
          disabled={!userCanUpdate()}
        />
      </Flex>
    </Show>
  );
};

export default DockerAccount;
