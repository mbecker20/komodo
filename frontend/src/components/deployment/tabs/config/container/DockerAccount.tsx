import { Component, createEffect, createSignal, Show } from "solid-js";
import { client } from "../../../../..";
import { useAppState } from "../../../../../state/StateProvider";
import { combineClasses } from "../../../../../util/helpers";
import Flex from "../../../../shared/layout/Flex";
import Selector from "../../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const DockerAccount: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const [dockerAccounts, setDockerAccounts] = createSignal<string[]>();
  createEffect(() => {
    client
      .get_server_docker_accounts(deployment.server_id)
      .then(setDockerAccounts);
  });
  return (
    <Flex
      class={combineClasses("config-item shadow")}
      justifyContent="space-between"
      alignItems="center"
      style={{ "flex-wrap": "wrap" }}
    >
      <h1>docker account</h1>
      <Selector
        targetClass="blue"
        items={["none", ...dockerAccounts()!]}
        selected={deployment.docker_run_args.docker_account || "none"}
        onSelect={(account) =>
          setDeployment("docker_run_args", {
            docker_account: account === "none" ? undefined : account,
          })
        }
        position="bottom right"
        disabled={!userCanUpdate()}
      />
    </Flex>
  );
};

export default DockerAccount;
