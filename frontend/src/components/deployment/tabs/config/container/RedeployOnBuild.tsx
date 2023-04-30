import { Component, Show } from "solid-js";
import { useConfig } from "../Provider";
import Flex from "../../../../shared/layout/Flex";

const RedeployOnBuild: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  return (
    <Show when={deployment.build_id}>
      <Flex
        class="config-item shadow"
        justifyContent="space-between"
        alignItems="center"
      >
        <h1>redeploy on build</h1>
        <Show
          when={userCanUpdate()}
          fallback={<h2>{deployment.redeploy_on_build ? "yes" : "no"}</h2>}
        >
          <button
            class={deployment.redeploy_on_build ? "green" : "red"}
            onClick={() => setDeployment("redeploy_on_build", (v) => !v)}
          >
            {deployment.redeploy_on_build ? "yes" : "no"}
          </button>
        </Show>
      </Flex>
    </Show>
  );
};

export default RedeployOnBuild;
