import { Component, Show } from "solid-js";
import { useAppState } from "../../../../state/StateProvider";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Selector from "../../../util/menu/Selector";
import { useConfig } from "./Provider";

const Git: Component<{}> = (p) => {
	const { githubAccounts } = useAppState();
	const { deployment, setDeployment } = useConfig();
	return (
    <Grid class="config-item shadow">
      <h1>deployment repo</h1>
      <Flex justifyContent="space-between" alignItems="center">
        <div>repo</div>
        <Input
          placeholder="ie. solidjs/solid"
          value={deployment.repo || ""}
          onEdit={(value) => setDeployment("repo", value)}
        />
      </Flex>
      <Flex justifyContent="space-between" alignItems="center">
        <div>branch</div>
        <Input
          placeholder="defaults to main"
          value={deployment.branch || ""}
          onEdit={(value) => setDeployment("branch", value)}
        />
      </Flex>
      <Show when={githubAccounts() && githubAccounts()!.length > 0}>
        <Flex justifyContent="space-between" alignItems="center">
          <div>github account</div>
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
          />
        </Flex>
      </Show>
    </Grid>
  );
}

export default Git;