import { ContainerStatus, DeployActionState } from "@monitor/types";
import {
  Component,
  createEffect,
  Match,
  onCleanup,
  Show,
  Switch,
} from "solid-js";
import { createStore } from "solid-js/store";
import { pushNotification } from "../..";
import {
  DELETE_CONTAINER,
  DEPLOY,
  START_CONTAINER,
  STOP_CONTAINER,
} from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { getDeploymentActionState } from "../../util/query";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/icons/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import Loading from "../util/loading/Loading";
import { useActionStates } from "./ActionStateProvider";

const Actions: Component<{}> = (p) => {
  const { ws, deployments, selected } = useAppState();
  const { permissions, username } = useUser();
  const deployment = () => deployments.get(selected.id())!;
  const actions = useActionStates();
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
                <Show
                  when={!actions.deploying}
                  fallback={
                    <button class="green">
                      <Loading type="spinner" />
                    </button>
                  }
                >
                  <ConfirmButton
                    color="green"
                    onConfirm={() => {
                      ws.send(DEPLOY, { deploymentID: deployment()._id });
                      pushNotification(
                        "ok",
                        `deploying ${deployment().name}...`
                      );
                    }}
                  >
                    <Icon type="reset" />
                  </ConfirmButton>
                </Show>

                <Show
                  when={!actions.deleting}
                  fallback={
                    <button class="red">
                      <Loading type="spinner" />
                    </button>
                  }
                >
                  <ConfirmButton
                    color="red"
                    onConfirm={() => {
                      ws.send(DELETE_CONTAINER, {
                        deploymentID: deployment()._id,
                      });
                      pushNotification("ok", `removing container...`);
                    }}
                  >
                    <Icon type="trash" />
                  </ConfirmButton>
                </Show>
              </Flex>
            </Flex>
            <Flex class="action shadow">
              container{" "}
              <Show
                when={!actions.stopping}
                fallback={
                  <button class="orange">
                    <Loading type="spinner" />
                  </button>
                }
              >
                <ConfirmButton
                  color="orange"
                  onConfirm={() => {
                    ws.send(STOP_CONTAINER, { deploymentID: deployment()._id });
                    pushNotification("ok", `stopping container`);
                  }}
                >
                  <Icon type="pause" />
                </ConfirmButton>
              </Show>
            </Flex>
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
                <Show
                  when={!actions.deploying}
                  fallback={
                    <button class="green">
                      <Loading type="spinner" />
                    </button>
                  }
                >
                  <ConfirmButton
                    color="green"
                    onConfirm={() => {
                      ws.send(DEPLOY, { deploymentID: deployment()._id });
                      pushNotification(
                        "ok",
                        `deploying ${deployment().name}...`
                      );
                    }}
                  >
                    <Icon type="reset" />
                  </ConfirmButton>
                </Show>
                <Show
                  when={!actions.deleting}
                  fallback={
                    <button class="red">
                      <Loading type="spinner" />
                    </button>
                  }
                >
                  <ConfirmButton
                    color="red"
                    onConfirm={() => {
                      ws.send(DELETE_CONTAINER, {
                        deploymentID: deployment()._id,
                      });
                      pushNotification("ok", `removing container...`);
                    }}
                  >
                    <Icon type="trash" />
                  </ConfirmButton>
                </Show>
              </Flex>
            </Flex>
            <Flex class="action shadow">
              container{" "}
              <Show
                when={!actions.starting}
                fallback={
                  <button class="green">
                    <Loading type="spinner" />
                  </button>
                }
              >
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
              </Show>
            </Flex>
          </Match>

          <Match when={deployment().status === "not deployed"}>
            <Flex class="action shadow">
              deploy{" "}
              <Show
                when={!actions.deploying}
                fallback={
                  <button class="green">
                    <Loading type="spinner" />
                  </button>
                }
              >
                <ConfirmButton
                  color="green"
                  onConfirm={() => {
                    ws.send(DEPLOY, { deploymentID: deployment()._id });
                    pushNotification("ok", `deploying ${deployment().name}...`);
                  }}
                >
                  <Icon type="play" />
                </ConfirmButton>
              </Show>
            </Flex>
          </Match>
        </Switch>
      </Grid>
    </Show>
  );
};

export default Actions;
