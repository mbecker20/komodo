import { Component } from "solid-js";
import { combineClasses } from "../../../../../util/helpers";
import Flex from "../../../../shared/layout/Flex";
import Selector from "../../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const Network: Component<{}> = (p) => {
  const { deployment, setDeployment, networks, userCanUpdate } = useConfig();
  return (
    <Flex
      class={combineClasses("config-item shadow")}
      justifyContent="space-between"
      alignItems="center"
    >
      <h1>network</h1>
      <Selector
        targetClass="blue"
        items={networks().map((net) => net.Name)}
        selected={deployment.docker_run_args.network || "bridge"}
        onSelect={(network) => setDeployment("docker_run_args", { network })}
        position="bottom right"
        disabled={!userCanUpdate()}
        searchStyle={{ width: "100%", "min-width": "12rem" }}
        useSearch
      />
    </Flex>
  );
};

export default Network;
