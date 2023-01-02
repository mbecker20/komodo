import { Component, createEffect, createSignal, Show } from "solid-js";
import { client } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../shared/Input";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import Selector from "../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const Docker: Component<{}> = (p) => {
  const { build, setBuild, userCanUpdate } = useConfig();
  const [dockerAccounts, setDockerAccounts] = createSignal<string[]>();
  createEffect(() => {
    client.get_server_docker_accounts(build.server_id).then(setDockerAccounts);
  });
  return (
    <Grid class={combineClasses("config-item shadow")}>
      <h1>docker build</h1> {/* checkbox here? */}
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>build path: </h2>
        <Input
          placeholder="from root of repo"
          value={build.docker_build_args?.build_path || ""}
          onEdit={(build_path) => setBuild("docker_build_args", { build_path })}
          disabled={!userCanUpdate()}
        />
      </Flex>
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>dockerfile path: </h2>
        <Input
          placeholder="from root of build path"
          value={
            build.docker_build_args?.dockerfile_path ||
            (userCanUpdate() ? "" : "Dockerfile")
          }
          onEdit={(dockerfile_path) =>
            setBuild("docker_build_args", { dockerfile_path })
          }
          disabled={!userCanUpdate()}
        />
      </Flex>
      <Show when={dockerAccounts() && dockerAccounts()!.length > 0}>
        <Flex
          justifyContent={userCanUpdate() ? "space-between" : undefined}
          alignItems="center"
          style={{ "flex-wrap": "wrap" }}
        >
          <h2>dockerhub account: </h2>
          <Selector
            targetClass="blue"
            selected={build.docker_account || "none"}
            items={["none", ...dockerAccounts()!]}
            onSelect={(account) => {
              setBuild(
                "docker_account",
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

export default Docker;
