import { ContainerStatus } from "@monitor/types";
import { Component, Match, Show, Switch } from "solid-js";
import { pushNotification } from "../..";
import {
  DELETE_CONTAINER,
  DEPLOY,
  PULL_DEPLOYMENT,
  START_CONTAINER,
  STOP_CONTAINER,
} from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import Loading from "../util/loading/Loading";
import HoverMenu from "../util/menu/HoverMenu";
import { useActionStates } from "./ActionStateProvider";

const Actions: Component<{}> = (p) => {
  const { deployments, selected } = useAppState();
  const { permissions, username } = useUser();
  const deployment = () => deployments.get(selected.id())!;
  return (
    <Show
      when={
        deployment() &&
        (permissions() >= 2 || deployment().owners.includes(username()!))
      }
    >
      <Grid class="card shadow">
        <h1>actions</h1>
        <Switch>
          <Match
            when={(deployment().status as ContainerStatus)?.State === "running"}
          >
            <Flex class="action shadow">
              deploy{" "}
              <Flex>
                <Deploy redeploy />
                <Stop />
                <Delete />
              </Flex>
            </Flex>
            {/* <Flex class="action shadow">
              container <Stop />
            </Flex> */}
          </Match>

          <Match
            when={
              (deployment().status as ContainerStatus).State === "exited" ||
              (deployment().status as ContainerStatus).State === "created"
            }
          >
            <Flex class="action shadow">
              deploy{" "}
              <Flex>
                <Deploy redeploy />
                <Start />
                <Delete />
              </Flex>
            </Flex>
            {/* <Flex class="action shadow">
              container <Start />
            </Flex> */}
          </Match>

          <Match when={deployment().status === "not deployed"}>
            <Flex class="action shadow">
              deploy <Deploy />
            </Flex>
          </Match>
        </Switch>
        <Show when={deployment().repo}>
          <Flex class="action shadow">
            pull <Pull />
          </Flex>
        </Show>
      </Grid>
    </Show>
  );
};

const Deploy: Component<{ redeploy?: boolean }> = (p) => {
  const { ws, deployments, selected } = useAppState();
  const deployment = () => deployments.get(selected.id())!;
  const actions = useActionStates();
  return (
    <Show
      when={!actions.deploying}
      fallback={
        <button class="green">
          <Loading type="spinner" />
        </button>
      }
    >
      <HoverMenu
        target={
          <ConfirmButton
            color="green"
            onConfirm={() => {
              ws.send(DEPLOY, { deploymentID: selected.id() });
              pushNotification("ok", `deploying ${deployment().name}...`);
            }}
          >
            <Icon type={p.redeploy ? "reset" : "play"} />
          </ConfirmButton>
        }
        content={p.redeploy ? "redeploy container" : "deploy container"}
        position="bottom center"
        padding="0.5rem"
      />
    </Show>
  );
};

const Delete = () => {
  const { ws, selected } = useAppState();
  const actions = useActionStates();
  return (
    <Show
      when={!actions.deleting}
      fallback={
        <button class="red">
          <Loading type="spinner" />
        </button>
      }
    >
      <HoverMenu
        target={
          <ConfirmButton
            color="red"
            onConfirm={() => {
              ws.send(DELETE_CONTAINER, {
                deploymentID: selected.id(),
              });
              pushNotification("ok", `removing container...`);
            }}
          >
            <Icon type="trash" />
          </ConfirmButton>
        }
        content="delete container"
        position="bottom center"
        padding="0.5rem"
      />
    </Show>
  );
};

const Start = () => {
  const { ws, deployments, selected } = useAppState();
  const deployment = () => deployments.get(selected.id())!;
  const actions = useActionStates();
  return (
    <Show
      when={!actions.starting}
      fallback={
        <button class="green">
          <Loading type="spinner" />
        </button>
      }
    >
      <HoverMenu
        target={
          <ConfirmButton
            color="green"
            onConfirm={() => {
              ws.send(START_CONTAINER, {
                deploymentID: deployment()._id,
              });
              pushNotification("ok", `starting container`);
            }}
          >
            <Icon type="play" />
          </ConfirmButton>
        }
        content="start container"
        position="bottom center"
        padding="0.5rem"
      />
    </Show>
  );
};

const Stop = () => {
  const { ws, deployments, selected } = useAppState();
  const deployment = () => deployments.get(selected.id())!;
  const actions = useActionStates();
  return (
    <Show
      when={!actions.stopping}
      fallback={
        <button class="orange">
          <Loading type="spinner" />
        </button>
      }
    >
      <HoverMenu
        target={
          <ConfirmButton
            color="orange"
            onConfirm={() => {
              ws.send(STOP_CONTAINER, {
                deploymentID: deployment()._id,
              });
              pushNotification("ok", `stopping container`);
            }}
          >
            <Icon type="pause" />
          </ConfirmButton>
        }
        content="stop container"
        position="bottom center"
        padding="0.5rem"
      />
    </Show>
  );
};

const Pull = () => {
  const { ws, deployments, selected } = useAppState();
  const deployment = () => deployments.get(selected.id())!;
  const actions = useActionStates();
  return (
    <Show
      when={!actions.pulling}
      fallback={
        <button class="blue">
          <Loading type="spinner" />
        </button>
      }
    >
      <HoverMenu
        target={
          <ConfirmButton
            color="blue"
            onConfirm={() => {
              ws.send(PULL_DEPLOYMENT, { deploymentID: selected.id() });
              pushNotification("ok", `pulling ${deployment().name}...`);
            }}
          >
            <Icon type="arrow-down" />
          </ConfirmButton>
        }
        content="pull repo"
        position="bottom center"
        padding="0.5rem"
      />
    </Show>
  );
};

export default Actions;
