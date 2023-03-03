import { Component, createEffect, createSignal, Show } from "solid-js";
import Grid from "../../../shared/layout/Grid";
import { useConfig } from "../Provider";
import Flex from "../../../shared/layout/Flex";
import Input from "../../../shared/Input";
import { combineClasses } from "../../../../util/helpers";
import { useAppState } from "../../../../state/StateProvider";
import { client } from "../../../..";
import { ServerStatus } from "../../../../types";
import Selector from "../../../shared/menu/Selector";

const Repo: Component<{}> = (p) => {
  const { aws_builder_config } = useAppState();
  const { build, setBuild, server, userCanUpdate } = useConfig();
  const [peripheryGithubAccounts, setPeripheryGithubAccounts] =
    createSignal<string[]>();
  createEffect(() => {
    if (server()?.status === ServerStatus.Ok) {
      client
        .get_server_github_accounts(build.server_id!)
        .then(setPeripheryGithubAccounts);
    }
  });
  const githubAccounts = () => {
    if (build.server_id) {
      return peripheryGithubAccounts() || [];
    } else if (build.aws_config) {
      const ami_name =
        build.aws_config?.ami_name || aws_builder_config()?.default_ami_name;
      return ami_name
        ? aws_builder_config()?.available_ami_accounts![ami_name].github || []
        : [];
    } else return [];
  };
  return (
    <Grid class={combineClasses("config-item shadow")}>
      <h1>repo config</h1>
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
      <Show when={githubAccounts()}>
        <Flex
          justifyContent={userCanUpdate() ? "space-between" : undefined}
          alignItems="center"
          style={{ "flex-wrap": "wrap" }}
        >
          <h2>github account: </h2>
          <Selector
            targetClass="blue"
            selected={build.github_account || "none"}
            items={["none", ...githubAccounts()]}
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
      </Show>
    </Grid>
  );
};

export default Repo;
