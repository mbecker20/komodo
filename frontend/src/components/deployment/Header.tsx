import { ContainerStatus } from "@monitor/types";
import { Component, Show } from "solid-js";
import { DELETE_DEPLOYMENT } from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { deploymentStatusClass } from "../../util/helpers";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import HoverMenu from "../util/menu/HoverMenu";
import { useActionStates } from "./ActionStateProvider";

const Header: Component<{}> = (p) => {
  const { servers, deployments, ws, selected } = useAppState();
  const deployment = () => deployments.get(selected.id());
  const server = () => deployment() && servers.get(deployment()?.serverID!);
  const state = () =>
    deployment()!.status === "not deployed"
      ? "not deployed"
      : (deployment()!.status as ContainerStatus).State;
  const status = () =>
    deployment()!.status === "not deployed"
      ? undefined
      : (deployment()!.status as ContainerStatus).Status.toLowerCase();
  const actions = useActionStates();
  return (
    <Grid gap="0.5rem" class="card shadow">
      <Flex alignItems="center" justifyContent="space-between">
        <h1>{deployment()!.name}</h1>
        <Show
          when={!actions.fullDeleting}
          fallback={
            <button class="red">
              <Icon type="trash" />
            </button>
          }
        >
          <HoverMenu
            target={
              <ConfirmButton
                onConfirm={() => {
                  ws.send(DELETE_DEPLOYMENT, { deploymentID: selected.id() });
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
        <HoverMenu
          target={
            <button
              class="grey"
              style={{ opacity: 0.8 }}
              onClick={() => selected.set(deployment()?.serverID!, "server")}
            >
              {server()!.name}
            </button>
          }
          content="show server"
          position="bottom center"
          padding="0.5rem"
        />
        <Flex alignItems="center">
          <div class={deploymentStatusClass(state())}>{state()}</div>
          <Show when={status()}>
            <div style={{ opacity: 0.7 }}>{status()}</div>
          </Show>
        </Flex>
      </Flex>
    </Grid>
  );
  return (
    <Flex
      class="card shadow"
      justifyContent="space-between"
      alignItems="center"
    >
      <Grid gap="0.1rem">
        <h1>{deployment()!.name}</h1>
        <button
          class="grey"
          style={{ opacity: 0.8 }}
          onClick={() => selected.set(deployment()?.serverID!, "server")}
        >
          {server()!.name}
        </button>
      </Grid>
      <Flex alignItems="center">
        <Grid gap="0.3rem" placeItems="center start">
          <div class={deploymentStatusClass(state())}>{state()}</div>
          <Show when={status()}>
            <div style={{ opacity: 0.7 }}>{status()}</div>
          </Show>
        </Grid>
        <Show
          when={!actions.fullDeleting}
          fallback={
            <button class="red">
              <Icon type="trash" />
            </button>
          }
        >
          <HoverMenu
            target={
              <ConfirmButton
                onConfirm={() => {
                  ws.send(DELETE_DEPLOYMENT, { deploymentID: selected.id() });
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
    </Flex>
  );
};

export default Header;
