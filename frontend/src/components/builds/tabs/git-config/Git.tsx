import { Component, Show } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Grid from "../../../util/layout/Grid";
import { useConfig } from "../Provider";
import Flex from "../../../util/layout/Flex";
import Input from "../../../util/Input";
import { useAppState } from "../../../../state/StateProvider";
import Selector from "../../../util/menu/Selector";

const Git: Component<{}> = (p) => {
  const { githubAccounts } = useAppState();
  const { build, setBuild } = useConfig();
  return (
    <Grid class="config-item shadow">
      <h1>github config</h1>
      <Flex justifyContent="space-between" alignItems="center">
        <div>repo</div>
        <Input
          placeholder="ie. solidjs/solid"
          value={build.repo || ""}
          onEdit={(value) => setBuild("repo", value)}
        />
      </Flex>
      <Flex justifyContent="space-between" alignItems="center">
        <div>branch</div>
        <Input
          placeholder="defaults to main"
          value={build.branch || ""}
          onEdit={(value) => setBuild("branch", value)}
        />
      </Flex>
      <Show when={githubAccounts() && githubAccounts()!.length > 0}>
        <Flex justifyContent="space-between" alignItems="center">
          <div>github account</div>
          <Selector
            targetClass="blue"
            selected={build.dockerAccount || "none"}
            items={["none", ...githubAccounts()!]}
            onSelect={(account) => {
              setBuild(
                "githubAccount",
                account === "none" ? undefined : account
              );
            }}
            position="bottom right"
          />
        </Flex>
      </Show>
    </Grid>
  );
};

export default Git;
