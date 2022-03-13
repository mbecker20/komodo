import { Component, Show } from "solid-js";
import { getAuthProvider } from "../util/helpers";
import Flex from "./util/layout/Flex";
import Grid from "./util/layout/Grid";
import { User } from "@oauth2/types";
import { pushNotification } from "..";

const UserInfo: Component<{ user: User; logout: () => void }> = (p) => {
  return (
    <Grid style={{ "font-size": "2rem" }}>
      <div>provider: {getAuthProvider(p.user)}</div>
      <Flex alignItems="center">
        <div>username: {p.user.username}</div>
        <Show when={p.user.avatar}>
          <img src={p.user.avatar} style={{ width: "2rem", height: "2rem" }} />
        </Show>
      </Flex>
      <Show when={p.user.email}>
        <div>email: {p.user.email}</div>
      </Show>
      <button style={{ width: "100%" }} onClick={() => {
        p.logout();
        pushNotification("ok", "logged out");
      }}>
        logout
      </button>
    </Grid>
  );
};

export default UserInfo;
