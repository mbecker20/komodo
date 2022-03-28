import { Server } from "@monitor/types";
import { Component } from "solid-js";
import { REMOVE_SERVER } from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/icons/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import s from "./server.module.css";

const Header: Component<{}> = (p) => {
  const { servers, selected, ws } = useAppState();
  const server = () => servers.get(selected.id()) as Server;
  return (
    <Flex
      class={combineClasses(s.Card, "shadow")}
      justifyContent="space-between"
      alignItems="center"
    >
      <Grid gap="0.1rem">
        <h1>{server().name}</h1>
        <div style={{ opacity: 0.8 }}>{server().address}</div>
      </Grid>
      <Flex alignItems="center">
        <div>
          {server().enabled
            ? server().status === "OK"
              ? "OK"
              : "NOT OK"
            : "DISABLED"}
        </div>
        <ConfirmButton
          onConfirm={() => {
            ws.send(REMOVE_SERVER, { serverID: selected.id() });
          }}
          color="red"
        >
          <Icon type="trash" />
        </ConfirmButton>
      </Flex>
    </Flex>
  );
};

export default Header;
