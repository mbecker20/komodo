import { Component, createEffect, createSignal } from "solid-js";
import Grid from "../../../shared/layout/Grid";
import { useConfig } from "../Provider";
import Flex from "../../../shared/layout/Flex";
import Input from "../../../shared/Input";
import Selector from "../../../shared/menu/Selector";
import { combineClasses } from "../../../../util/helpers";
import { client } from "../../../..";
import { ServerStatus } from "../../../../types";

const Git: Component<{}> = (p) => {
  const { build, setBuild, server, userCanUpdate } = useConfig();
  const [githubAccounts, setGithubAccounts] = createSignal<string[]>();
  createEffect(() => {
    if (server()?.status === ServerStatus.Ok) {
      client.get_server_github_accounts(build.server_id).then(setGithubAccounts);
    }
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
          value={build.repo || ""}
          onEdit={(value) => setBuild("repo", value)}
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
          value={build.branch || (userCanUpdate() ? "" : "main")}
          onEdit={(value) => setBuild("branch", value)}
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
          selected={build.github_account || "none"}
          items={["none", ...githubAccounts()!]}
          onSelect={(account) => {
            setBuild(
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
