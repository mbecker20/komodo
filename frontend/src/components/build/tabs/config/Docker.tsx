import { Component, createEffect, createSignal, Show } from "solid-js";
import { client } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { ServerStatus } from "../../../../types";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../shared/Input";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import Selector from "../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const Docker: Component<{}> = (p) => {
  const { aws_builder_config } = useAppState();
  const { build, setBuild, server, userCanUpdate } = useConfig();
  const [peripheryDockerAccounts, setPeripheryDockerAccounts] =
    createSignal<string[]>();
  createEffect(() => {
    if (server()?.status === ServerStatus.Ok) {
      client
        .get_server_docker_accounts(build.server_id!)
        .then(setPeripheryDockerAccounts);
    }
  });
  const dockerAccounts = () => {
    if (build.server_id) {
      return peripheryDockerAccounts() || [];
    } else if (build.aws_config) {
      const ami_id =
        build.aws_config?.ami_id || aws_builder_config()?.default_ami_id;
      return ami_id
        ? aws_builder_config()?.available_ami_accounts![ami_id].docker || []
        : [];
    } else return [];
  };
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
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>dockerhub account: </h2>
        <Selector
          targetClass="blue"
          selected={build.docker_account || "none"}
          items={["none", ...dockerAccounts()]}
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
    </Grid>
  );
};

export default Docker;
