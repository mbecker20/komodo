import { Component, createEffect, createSignal } from "solid-js";
import { client } from "../../../../..";
import { combineClasses } from "../../../../../util/helpers";
import Input from "../../../../shared/Input";
import Flex from "../../../../shared/layout/Flex";
import Grid from "../../../../shared/layout/Grid";
import Selector from "../../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const Git: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const [githubAccounts, setGithubAccounts] = createSignal<string[]>();
  createEffect(() => {
    client.get_server_github_accounts(deployment.server_id).then(setGithubAccounts);
  });
  return (
    <Grid class={combineClasses("config-item shadow")}>
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
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>github account: </h2>
        <Selector
          targetClass="blue"
          selected={deployment.github_account || "none"}
          items={["none", ...githubAccounts()!]}
          onSelect={(account) => {
            setDeployment(
              "github_account",
              account === "none" ? undefined : account
            );
          }}
          position="bottom right"
          disabled={!userCanUpdate()}
        />
      </Flex>
    </Grid>
  );
};

export default Git;
