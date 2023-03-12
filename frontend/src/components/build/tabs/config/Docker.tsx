import {
  Component,
  createResource,
  Show,
} from "solid-js";
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
  const [dockerOrgs] = createResource(() => client.get_docker_organizations());
  const [peripheryDockerAccounts] = createResource(() => {
    if (server()?.status === ServerStatus.Ok) {
      return client.get_server_docker_accounts(build.server_id!);
    } else return [];
  });
  const dockerAccounts = () => {
    if (build.server_id) {
      return peripheryDockerAccounts() || [];
    } else if (build.aws_config) {
      const ami_name =
        build.aws_config?.ami_name || aws_builder_config()?.default_ami_name;
      return ami_name
        ? aws_builder_config()?.available_ami_accounts![ami_name].docker || []
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
      <Show when={build.docker_organization || (dockerOrgs() || []).length > 0}>
        <Flex
          justifyContent={userCanUpdate() ? "space-between" : undefined}
          alignItems="center"
          style={{ "flex-wrap": "wrap" }}
        >
          <h2>dockerhub organization: </h2>
          <Selector
            targetClass="blue"
            selected={build.docker_organization || "none"}
            items={["none", ...(dockerOrgs() || [])]}
            onSelect={(account) => {
              setBuild(
                "docker_organization",
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
