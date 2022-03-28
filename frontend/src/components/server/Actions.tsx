import { Component, Show } from "solid-js";
import { pushNotification } from "../..";
import { PRUNE_IMAGES, PRUNE_NETWORKS } from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/icons/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import s from "./server.module.css";

const Actions: Component<{}> = (p) => {
  const { ws, servers, selected } = useAppState();
  const server = () => servers.get(selected.id())!;
  return (
    <Show when={server() && server().status === "OK"}>
      <Grid class={combineClasses(s.Card, "shadow")}>
        <h1>actions</h1>
        <Flex class={combineClasses(s.Action, "shadow")}>
          prune images{" "}
          <ConfirmButton
            color="green"
            onConfirm={() => {
              ws.send(PRUNE_IMAGES, { serverID: server()._id });
              pushNotification("ok", `pruning images on ${server().name}...`);
            }}
          >
            <Icon type="cut" />
          </ConfirmButton>
        </Flex>
        <Flex class={combineClasses(s.Action, "shadow")}>
          prune networks{" "}
          <ConfirmButton
            color="green"
            onConfirm={() => {
              ws.send(PRUNE_NETWORKS, { serverID: server()._id });
              pushNotification("ok", `pruning networks on ${server().name}...`);
            }}
          >
            <Icon type="cut" />
          </ConfirmButton>
        </Flex>
      </Grid>
    </Show>
  );
};

export default Actions;
