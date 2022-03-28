import { ContainerStatus } from "@monitor/types";
import { Component, Match, Show, Switch } from "solid-js";
import { pushNotification } from "../..";
import {
  DELETE_CONTAINER,
  DEPLOY,
  START_CONTAINER,
  STOP_CONTAINER,
} from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/icons/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import s from "./deployment.module.css";

const Actions: Component<{}> = (p) => {
  const { ws, deployments, selected } = useAppState();
  const deployment = () => deployments.get(selected.id())!;
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
                  <Icon type="reset" />
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
                  <Icon type="trash" />
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
                <Icon type="pause" />
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
                  <Icon type="reset" />
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
                  <Icon type="trash" />
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
                <Icon type="play" />
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
                <Icon type="play" />
              </ConfirmButton>
            </Flex>
          </Match>
        </Switch>
      </Grid>
    </Show>
  );
};

export default Actions;
