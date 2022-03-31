import { Component, Show } from "solid-js";
import { useAppState } from "../../../../state/StateProvider";
import { combineClasses } from "../../../../util/helpers";
import Flex from "../../../util/layout/Flex";
import Selector from "../../../util/menu/Selector";
import s from "../../deployment.module.css";
import { useConfig } from "./Provider";

const DockerAccount: Component<{}> = (p) => {
	const { dockerAccounts } = useAppState();
  const { deployment, setDeployment } = useConfig();
  return (
    <Show when={dockerAccounts}>
      <Flex
        class={combineClasses(s.ConfigItem, "shadow")}
        justifyContent="space-between"
      >
        <h1>docker account</h1>
        <Selector
          items={["none", ...dockerAccounts()!]}
          selected={deployment.dockerAccount || "none"}
          onSelect={(account) =>
            setDeployment(
              "dockerAccount",
              account === "none" ? undefined : account
            )
          }
          position="bottom right"
        />
      </Flex>
    </Show>
  );
};

export default DockerAccount;
