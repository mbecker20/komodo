import { Component, Show } from "solid-js";
import { pushNotification } from "../..";
import { PRUNE_IMAGES, PRUNE_NETWORKS } from "@monitor/util";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import { useTheme } from "../../state/ThemeProvider";
import { combineClasses } from "../../util/helpers";

const Actions: Component<{}> = (p) => {
  const { ws, servers, selected } = useAppState();
  const { permissions } = useUser();
  const server = () => servers.get(selected.id())!;
  const { themeClass } = useTheme();
  return (
    <Show when={server() && server().status === "OK" && permissions() > 1}>
      <Grid class={combineClasses("card shadow", themeClass())}>
        <h1>actions</h1>
        <Flex class={combineClasses("action shadow", themeClass())}>
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
        <Flex class={combineClasses("action shadow", themeClass())}>
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
