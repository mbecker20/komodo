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
import { combineClasses } from "../../util/helpers";
import { getDeploymentActionState } from "../../util/query";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/icons/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import Loading from "../util/loading/Loading";
import s from "./deployment.module.css";

const Actions: Component<{}> = (p) => {
  const { ws, deployments, selected } = useAppState();
  const deployment = () => deployments.get(selected.id())!;
  const [actions, setActions] = createStore<DeployActionState>({
    deploying: false,
    deleting: false,
    starting: false,
    stopping: false,
  });
  createEffect(() => {
    getDeploymentActionState(selected.id()).then(setActions);
  });
  onCleanup(
    ws.subscribe([DEPLOY], ({ complete, deploymentID }) => {
      if (deploymentID === selected.id()) {
        setActions("deploying", !complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([DELETE_CONTAINER], ({ complete, deploymentID }) => {
      if (deploymentID === selected.id()) {
        setActions("deleting", !complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([START_CONTAINER], ({ complete, deploymentID }) => {
      if (deploymentID === selected.id()) {
        setActions("starting", !complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([STOP_CONTAINER], ({ complete, deploymentID }) => {
      if (deploymentID === selected.id()) {
        setActions("stopping", !complete);
      }
    })
  );
  return (
    <Show when={deployment()}>
      <Grid class={combineClasses(s.Card, "shadow")}>
        <h1>actions</h1>
        <Switch>
          <Match
            when={(deployment().status as ContainerStatus)?.State === "running"}
          >
            <Flex class={combineClasses(s.Action, "shadow")}>
              deploy{" "}
              <Flex>
                <ConfirmButton
                  color="green"
                  onConfirm={() => {
                    ws.send(DEPLOY, { deploymentID: deployment()._id });
                    pushNotification("ok", `deploying ${deployment().name}...`);
                  }}
                >
                  <Show
                    when={!actions.deploying}
                    fallback={<Loading type="spinner" scale={0.25} />}
                  >
                    <Icon type="reset" />
                  </Show>
                </ConfirmButton>
                <ConfirmButton
                  color="red"
                  onConfirm={() => {
                    ws.send(DELETE_CONTAINER, {
                      deploymentID: deployment()._id,
                    });
                    pushNotification("ok", `removing container...`);
                  }}
                >
                  <Show
                    when={!actions.deleting}
                    fallback={<Loading type="spinner" scale={0.25} />}
                  >
                    <Icon type="trash" />
                  </Show>
                </ConfirmButton>
              </Flex>
            </Flex>
            <Flex class={combineClasses(s.Action, "shadow")}>
              container{" "}
              <ConfirmButton
                color="orange"
                onConfirm={() => {
                  ws.send(STOP_CONTAINER, { deploymentID: deployment()._id });
                  pushNotification("ok", `stopping container`);
                }}
              >
                <Show
                  when={!actions.stopping}
                  fallback={<Loading type="spinner" scale={0.25} />}
                >
                  <Icon type="pause" />
                </Show>
              </ConfirmButton>
            </Flex>
          </Match>

          <Match
            when={
              (deployment().status as ContainerStatus).State === "exited" ||
              (deployment().status as ContainerStatus).State === "created"
            }
          >
            <Flex class={combineClasses(s.Action, "shadow")}>
              deploy{" "}
              <Flex>
                <ConfirmButton
                  color="green"
                  onConfirm={() => {
                    ws.send(DEPLOY, { deploymentID: deployment()._id });
                    pushNotification("ok", `deploying ${deployment().name}...`);
                  }}
                >
                  <Show
                    when={!actions.deploying}
                    fallback={<Loading type="spinner" scale={0.25} />}
                  >
                    <Icon type="reset" />
                  </Show>
                </ConfirmButton>
                <ConfirmButton
                  color="red"
                  onConfirm={() => {
                    ws.send(DELETE_CONTAINER, {
                      deploymentID: deployment()._id,
                    });
                    pushNotification("ok", `removing container...`);
                  }}
                >
                  <Show
                    when={!actions.deleting}
                    fallback={<Loading type="spinner" scale={0.25} />}
                  >
                    <Icon type="trash" />
                  </Show>
                </ConfirmButton>
              </Flex>
            </Flex>
            <Flex class={combineClasses(s.Action, "shadow")}>
              container{" "}
              <ConfirmButton
                color="green"
                onConfirm={() => {
                  ws.send(START_CONTAINER, { deploymentID: deployment()._id });
                  pushNotification("ok", `starting container...`);
                }}
              >
                <Show
                  when={!actions.starting}
                  fallback={<Loading type="spinner" scale={0.25} />}
                >
                  <Icon type="play" />
                </Show>
              </ConfirmButton>
            </Flex>
          </Match>

          <Match when={deployment().status === "not deployed"}>
            <Flex class={combineClasses(s.Action, "shadow")}>
              deploy{" "}
              <ConfirmButton
                color="green"
                onConfirm={() => {
                  ws.send(DEPLOY, { deploymentID: deployment()._id });
                  pushNotification("ok", `deploying ${deployment().name}...`);
                }}
              >
                <Show
                  when={!actions.deploying}
                  fallback={<Loading type="spinner" scale={0.25} />}
                >
                  <Icon type="play" />
                </Show>
              </ConfirmButton>
            </Flex>
          </Match>
        </Switch>
      </Grid>
    </Show>
  );
};

export default Actions;
