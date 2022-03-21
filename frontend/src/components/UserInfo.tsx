import { Component, Show } from "solid-js";
import { getAuthProvider } from "../util/helpers";
import Flex from "./util/layout/Flex";
import Grid from "./util/layout/Grid";
import { User } from "@monitor/types";
import { pushNotification } from "..";
import { useUser } from "../state/UserProvider";
import { useAppState } from "../state/StateProvider";

const UserInfo: Component<{}> = (p) => {
  const { user, logout } = useUser();
  const { ws } = useAppState();
  return (
    <Grid style={{ "font-size": "2rem" }}>
      <div>provider: {getAuthProvider(user() as User)}</div>
      <Flex alignItems="center">
        <div>username: {user().username}</div>
        <Show when={user().avatar}>
          <img src={user().avatar} style={{ width: "2rem", height: "2rem" }} />
        </Show>
      </Flex>
      <button style={{ width: "100%" }} onClick={() => {
        logout();
        ws.close();
        pushNotification("ok", "logged out");
      }}>
        logout
      </button>
    </Grid>
  );
};

export default UserInfo;
