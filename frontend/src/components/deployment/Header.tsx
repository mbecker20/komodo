import { Component, createResource, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import {
  combineClasses,
  deploymentHeaderStateClass,
  getId,
  readableVersion,
} from "../../util/helpers";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import HoverMenu from "../shared/menu/HoverMenu";
import { useLocalStorageToggle } from "../../util/hooks";
import { useAppDimensions } from "../../state/DimensionProvider";
import Updates from "./Updates";
import { DockerContainerState, PermissionLevel } from "../../types";
import { A, useParams } from "@solidjs/router";
import { client } from "../..";
import CopyMenu from "../CopyMenu";
import ConfirmMenuButton from "../shared/ConfirmMenuButton";

const Header: Component<{}> = (p) => {
  const { deployments, servers, builds } = useAppState();
  const params = useParams();
  const deployment = () => deployments.get(params.id)!;
  const { user } = useUser();
  const status = () =>
    deployment()!.state === DockerContainerState.Unknown ||
    deployment()!.state === DockerContainerState.NotDeployed
      ? undefined
      : deployment().container?.status?.toLowerCase();
  const { isSemiMobile } = useAppDimensions();
  const [showUpdates, toggleShowUpdates] =
    useLocalStorageToggle("show-updates");
  const userCanUpdate = () =>
    user().admin ||
    deployment().deployment.permissions![getId(user())] ===
      PermissionLevel.Update;
  const server = () => servers.get(deployment().deployment.server_id);
  const [deployed_version] = createResource(() =>
    client.get_deployment_deployed_version(params.id)
  );
  const image = () => {
    if (deployment().deployment.build_id) {
      const build = builds.get(deployment().deployment.build_id!)!;
      if (deployment().state === DockerContainerState.NotDeployed) {
        const version = deployment().deployment.build_version
          ? readableVersion(deployment().deployment.build_version!).replaceAll(
              "v",
              ""
            )
          : "latest";
        return `${build.name}:${version}`;
      } else {
        return deployed_version() && `${build.name}:${deployed_version()}`;
      }
    } else {
      return deployment().deployment.docker_run_args.image || "unknown";
    }
  };
  return (
    <>
      <Grid
        gap="0.5rem"
        class={combineClasses("card shadow")}
        style={{
          position: "relative",
          cursor: isSemiMobile() ? "pointer" : undefined,
          height: "fit-content",
        }}
        onClick={() => {
          if (isSemiMobile()) toggleShowUpdates();
        }}
      >
        <Flex alignItems="center" justifyContent="space-between">
          <Flex alignItems="center">
            <h1>{deployment()!.deployment.name}</h1>
            <div style={{ opacity: 0.7 }}>{image()}</div>
          </Flex>
          <Show when={userCanUpdate()}>
            <Flex alignItems="center">
              <CopyMenu type="deployment" id={params.id} />
              <HoverMenu
                target={
                  <ConfirmMenuButton
                    onConfirm={() => {
                      client.delete_deployment(params.id);
                    }}
                    class="red"
                    title="delete deployment"
                    match={deployment().deployment.name}
                    info={
                      <Show when={deployment().container}>
                        <div style={{ opacity: 0.7 }}>
                          warning! this will destroy this deployments container
                        </div>
                      </Show>
                    }
                  >
                    <Icon type="trash" />
                  </ConfirmMenuButton>
                }
                content="delete deployment"
                position="bottom center"
                padding="0.5rem"
              />
            </Flex>
          </Show>
        </Flex>
        <Flex alignItems="center" justifyContent="space-between">
          <Flex alignItems="center">
            <A
              href={`/server/${deployment().deployment.server_id}`}
              class="text-hover"
              style={{ opacity: 0.7, padding: 0 }}
            >
              {server()?.server.name}
            </A>
            <div class={deploymentHeaderStateClass(deployment().state)}>
              {deployment().state}
            </div>
          </Flex>
          <Show when={status()}>
            <div style={{ opacity: 0.7 }}>{status()}</div>
          </Show>
        </Flex>
        <Show when={isSemiMobile()}>
          <Flex gap="0.5rem" alignItems="center" class="show-updates-indicator">
            updates{" "}
            <Icon
              type={showUpdates() ? "chevron-up" : "chevron-down"}
              width="0.9rem"
            />
          </Flex>
        </Show>
      </Grid>
      <Show when={isSemiMobile() && showUpdates()}>
        <Updates />
      </Show>
    </>
  );
};

export default Header;
