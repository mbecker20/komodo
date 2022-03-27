import { ContainerStatus, Deployment } from "@monitor/types";
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

const Actions: Component<{ deployment: Deployment }> = (p) => {
  const { ws } = useAppState();
  return (
    <Show when={true}>
      <Grid class={combineClasses(s.Actions, "shadow")}>
        <div class={s.ItemHeader}>actions</div>
        <Switch>
          <Match
            when={(p.deployment.status as ContainerStatus)?.State === "running"}
          >
            <Flex class={combineClasses(s.Action, "shadow")}>
              deploy{" "}
              <Flex>
                <ConfirmButton
                  color="green"
                  onConfirm={() => {
                    ws.send(DEPLOY, { deploymentID: p.deployment._id });
                    pushNotification("ok", `deploying ${p.deployment.name}...`);
                  }}
                >
                  <Icon type="reset" />
                </ConfirmButton>
                <ConfirmButton
                  color="red"
                  onConfirm={() => {
                    ws.send(DELETE_CONTAINER, {
                      deploymentID: p.deployment._id,
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
                  ws.send(STOP_CONTAINER, { deploymentID: p.deployment._id });
                  pushNotification("ok", `stopping container`);
                }}
              >
                <Icon type="pause" />
              </ConfirmButton>
            </Flex>
          </Match>

          <Match
            when={(p.deployment.status as ContainerStatus).State === "exited"}
          >
            <Flex class={combineClasses(s.Action, "shadow")}>
              deploy{" "}
              <Flex>
                <ConfirmButton
                  color="green"
                  onConfirm={() => {
                    ws.send(DEPLOY, { deploymentID: p.deployment._id });
                    pushNotification("ok", `deploying ${p.deployment.name}...`);
                  }}
                >
                  <Icon type="reset" />
                </ConfirmButton>
                <ConfirmButton
                  color="red"
                  onConfirm={() => {
                    ws.send(DELETE_CONTAINER, {
                      deploymentID: p.deployment._id,
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
                  ws.send(START_CONTAINER, { deploymentID: p.deployment._id });
                  pushNotification("ok", `starting container...`);
                }}
              >
                <Icon type="play" />
              </ConfirmButton>
            </Flex>
          </Match>

          <Match when={p.deployment.status === "not created"}>
            <Flex class={combineClasses(s.Action, "shadow")}>
              deploy{" "}
              <ConfirmButton
                color="green"
                onConfirm={() => {
                  ws.send(DEPLOY, { deploymentID: p.deployment._id });
                  pushNotification("ok", `deploying ${p.deployment.name}...`);
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
