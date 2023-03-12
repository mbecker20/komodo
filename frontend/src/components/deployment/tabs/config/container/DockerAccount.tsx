import { Component } from "solid-js";
import { useAppState } from "../../../../../state/StateProvider";
import { ServerStatus } from "../../../../../types";
import { combineClasses } from "../../../../../util/helpers";
import Flex from "../../../../shared/layout/Flex";
import Selector from "../../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const DockerAccount: Component<{}> = (p) => {
  const { serverDockerAccounts } = useAppState();
  const { deployment, setDeployment, server, userCanUpdate } = useConfig();
  const dockerAccounts = () =>
    serverDockerAccounts.get(
      deployment.server_id,
      server()?.status || ServerStatus.NotOk
    ) || [];
  const when_none_selected = () => {
    if (deployment.build_id) {
      return "same as build";
    } else {
      return "none";
    }
  };
  const accounts = () => [when_none_selected(), ...dockerAccounts()];
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
        items={accounts()}
        selected={
          deployment.docker_run_args.docker_account || when_none_selected()
        }
        onSelect={(account) =>
          setDeployment("docker_run_args", {
            docker_account:
              account === when_none_selected() ? undefined : account,
          })
        }
        position="bottom right"
        disabled={!userCanUpdate()}
      />
    </Flex>
  );
};

export default DockerAccount;
