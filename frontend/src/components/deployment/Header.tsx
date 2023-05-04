import { Component, createResource, createSignal, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import {
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
import Loading from "../shared/loading/Loading";
import { AutofocusInput } from "../shared/Input";

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
  const derived_image = () => {
    if (deployment().deployment.build_id) {
      const build = builds.get(deployment().deployment.build_id!);
      if (build === undefined) return "unknown";
      const version =
        deployment().state === DockerContainerState.NotDeployed
          ? deployment().deployment.build_version
            ? readableVersion(
                deployment().deployment.build_version!
              ).replaceAll("v", "")
            : "latest"
          : deployed_version() || "unknown";
      return `${build.name}:${version}`;
    } else {
      return deployment().deployment.docker_run_args.image || "unknown";
    }
  };
  const image = () => {
    if (deployment().state === DockerContainerState.NotDeployed) {
      return derived_image();
    } else if (deployment().container?.image) {
      if (deployment().container!.image.includes("sha256:")) {
        return derived_image();
      }
      let [account, image] = deployment().container!.image.split("/");
      return image ? image : account;
    } else {
      return "unknown";
    }
  };
  const [editingName, setEditingName] = createSignal(false);
  const [updatingName, setUpdatingName] = createSignal(false);
  return (
    <>
      <Grid
        gap="0.5rem"
        class="card shadow"
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
            <Show
              when={editingName()}
              fallback={
                <button
                  onClick={() => setEditingName(true)}
                  style={{ padding: 0 }}
                >
                  <h1>{deployment()!.deployment.name}</h1>
                </button>
              }
            >
              <Show
                when={!updatingName()}
                fallback={<Loading type="three-dot" />}
              >
                <AutofocusInput
                  value={deployment().deployment.name}
                  placeholder={deployment().deployment.name}
                  onEnter={async (new_name) => {
                    setUpdatingName(true);
                    await client.rename_deployment(params.id, new_name);
                    setEditingName(false);
                    setUpdatingName(false);
                  }}
                  onBlur={() => setEditingName(false)}
                  style={{ "font-size": "1.4rem" }}
                />
              </Show>
            </Show>
            <Show
              when={deployment().deployment.build_id}
              fallback={<div style={{ opacity: 0.7 }}>{image()}</div>}
            >
              <A
                href={`/build/${deployment().deployment.build_id}`}
                class="text-hover"
                style={{ opacity: 0.7, padding: 0 }}
              >
                {image()}
              </A>
            </Show>
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
              {server()?.server.name || "unknown"}
            </A>
            <div class={deploymentHeaderStateClass(deployment().state)}>
              {deployment().state.replaceAll("_", " ")}
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
