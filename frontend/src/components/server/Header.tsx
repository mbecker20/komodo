import { Server } from "@monitor/types";
import { Component, Show } from "solid-js";
import { REMOVE_SERVER } from "@monitor/util";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { combineClasses, serverStatusClass } from "../../util/helpers";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import { useTheme } from "../../state/ThemeProvider";

const Header: Component<{}> = (p) => {
  const { servers, selected, ws } = useAppState();
  const server = () => servers.get(selected.id()) as Server;
  const status = () =>
    server().enabled
      ? server().status === "OK"
        ? "OK"
        : "NOT OK"
      : "DISABLED";
  const { permissions, username } = useUser();
  const { themeClass } = useTheme();
  return (
    <Flex
      class={combineClasses("card shadow", themeClass())}
      justifyContent="space-between"
      alignItems="center"
    >
      <Grid gap="0.1rem">
        <h1>{server().name}</h1>
        <div style={{ opacity: 0.8 }}>{server().address}</div>
      </Grid>
      <Show when={!server().isCore}>
        <Flex alignItems="center">
          <div class={serverStatusClass(status())}>{status()}</div>
          <Show
            when={permissions() > 1 || server().owners.includes(username())}
          >
            <ConfirmButton
              onConfirm={() => {
                ws.send(REMOVE_SERVER, { serverID: selected.id() });
              }}
              color="red"
            >
              <Icon type="trash" />
            </ConfirmButton>
          </Show>
        </Flex>
      </Show>
    </Flex>
  );
};

export default Header;
