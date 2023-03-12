import { Component, createResource } from "solid-js";
import { useAppState } from "../../../../../state/StateProvider";
import { ServerStatus } from "../../../../../types";
import { combineClasses } from "../../../../../util/helpers";
import Input from "../../../../shared/Input";
import Flex from "../../../../shared/layout/Flex";
import Grid from "../../../../shared/layout/Grid";
import Selector from "../../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const Git: Component<{}> = (p) => {
  const { serverGithubAccounts } = useAppState();
  const { deployment, server, setDeployment, userCanUpdate } = useConfig();
  const githubAccounts = () =>
    serverGithubAccounts.get(
      deployment.server_id,
      server()?.status || ServerStatus.NotOk
    ) || [];
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
          items={["none", ...githubAccounts()]}
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
