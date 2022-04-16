import { Component, Show } from "solid-js";
import { useAppState } from "../../../../../state/StateProvider";
import Input from "../../../../util/Input";
import Flex from "../../../../util/layout/Flex";
import Grid from "../../../../util/layout/Grid";
import Selector from "../../../../util/menu/Selector";
import { useConfig } from "../Provider";

const Git: Component<{}> = (p) => {
  const { githubAccounts } = useAppState();
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  return (
    <Grid class="config-item shadow">
      <h1>github config</h1>
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>repo: </h2>
        <Input
          placeholder="ie. solidjs/solid"
          value={deployment.repo || ""}
          onEdit={(value) => setDeployment("repo", value)}
          disabled={!userCanUpdate()}
        />
      </Flex>
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>branch: </h2>
        <Input
          placeholder="defaults to main"
          value={deployment.branch || ""}
          onEdit={(value) => setDeployment("branch", value)}
          disabled={!userCanUpdate()}
        />
      </Flex>
      <Show when={githubAccounts() && githubAccounts()!.length > 0}>
        <Flex
          justifyContent={userCanUpdate() ? "space-between" : undefined}
          alignItems="center"
          style={{ "flex-wrap": "wrap" }}
        >
          <h2>github account: </h2>
          <Selector
            targetClass="blue"
            selected={deployment.githubAccount || "none"}
            items={["none", ...githubAccounts()!]}
            onSelect={(account) => {
              setDeployment(
                "githubAccount",
                account === "none" ? undefined : account
              );
            }}
            position="bottom right"
            disabled={!userCanUpdate()}
          />
        </Flex>
      </Show>
    </Grid>
  );
};

export default Git;
