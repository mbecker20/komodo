import { Component, createEffect, Show } from "solid-js";
import Grid from "../../../util/layout/Grid";
import { useConfig } from "../Provider";
import Flex from "../../../util/layout/Flex";
import Input from "../../../util/Input";
import { useAppState } from "../../../../state/StateProvider";
import Selector from "../../../util/menu/Selector";

const Git: Component<{}> = (p) => {
  const { githubAccounts } = useAppState();
  const { build, setBuild, userCanUpdate } = useConfig();
  createEffect(() => console.log(build.branch));
  return (
    <Grid class="config-item shadow">
      <h1>github config</h1>
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
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
      >
        <h2>branch: </h2>
        <Input
          placeholder="defaults to main"
          value={build.branch || (userCanUpdate() ? "" : "main")}
          onEdit={(value) => setBuild("branch", value)}
          disabled={!userCanUpdate()}
        />
      </Flex>
      <Show when={githubAccounts() && githubAccounts()!.length > 0}>
        <Flex
          justifyContent={userCanUpdate() ? "space-between" : undefined}
          alignItems="center"
        >
          <h2>github account: </h2>
          <Selector
            targetClass="blue"
            selected={build.githubAccount || "none"}
            items={["none", ...githubAccounts()!]}
            onSelect={(account) => {
              setBuild(
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
