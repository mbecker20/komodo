import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { combineClasses, deploymentHeaderStateClass, getId } from "../../util/helpers";
import ConfirmButton from "../shared/ConfirmButton";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import HoverMenu from "../shared/menu/HoverMenu";
import { useLocalStorageToggle } from "../../util/hooks";
import { useAppDimensions } from "../../state/DimensionProvider";
import Updates from "./Updates";
import { DockerContainerState, PermissionLevel } from "../../types";
import { useParams } from "@solidjs/router";
import { client } from "../..";

const Header: Component<{}> = (p) => {
  const { deployments } = useAppState();
  const params = useParams();
  const deployment = () => deployments.get(params.id)!;
  const { user } = useUser();
  const status = () =>
    deployment()!.state === DockerContainerState.Unknown ||
    deployment()!.state === DockerContainerState.NotDeployed
      ? undefined
      : deployment().container?.status?.toLowerCase();
  const { isMobile } = useAppDimensions();
  const [showUpdates, toggleShowUpdates] =
    useLocalStorageToggle("show-updates");
  const userCanUpdate = () =>
    user().admin ||
    deployment().deployment.permissions![getId(user())] ===
      PermissionLevel.Update;
  return (
    <>
      <Grid
        gap="0.5rem"
        class={combineClasses("card shadow")}
        style={{
          position: "relative",
          cursor: isMobile() && userCanUpdate() ? "pointer" : undefined,
        }}
        onClick={() => {
          if (isMobile() && userCanUpdate()) toggleShowUpdates();
        }}
      >
        <Flex alignItems="center" justifyContent="space-between">
          <h1>{deployment()!.deployment.name}</h1>
          <Show when={userCanUpdate()}>
            <HoverMenu
              target={
                <ConfirmButton
                  onConfirm={() => {
                    client.delete_deployment(params.id)
                  }}
                  color="red"
                >
                  <Icon type="trash" />
                </ConfirmButton>
              }
              content="delete deployment"
              position="bottom center"
              padding="0.5rem"
            />
          </Show>
        </Flex>
        <Flex alignItems="center" justifyContent="space-between">
          <div class={deploymentHeaderStateClass(deployment().state)}>
            {deployment().state}
          </div>
          <Show when={status()}>
            <div style={{ opacity: 0.7 }}>{status()}</div>
          </Show>
        </Flex>
        <Show when={isMobile() && userCanUpdate()}>
          <Flex gap="0.5rem" alignItems="center" class="show-updates-indicator">
            updates{" "}
            <Icon
              type={showUpdates() ? "chevron-up" : "chevron-down"}
              width="0.9rem"
            />
          </Flex>
        </Show>
      </Grid>
      <Show when={isMobile() && userCanUpdate() && showUpdates()}>
        <Updates />
      </Show>
    </>
  );
};

export default Header;
