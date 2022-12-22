import { Component, Show } from "solid-js";
import { client } from "../..";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useUser } from "../../state/UserProvider";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import s from "./topbar.module.scss";

const Account: Component<{ close: () => void }> = (p) => {
  const { user } = useUser();
  const { isMobile } = useAppDimensions();
  return (
    <Grid gap="0.5rem" class={s.Account} placeItems="center end">
      <Show when={isMobile()}>
        <Flex justifyContent="center">{user().username}</Flex>
      </Show>
      <Flex justifyContent="center">admin: {user().admin.toString()}</Flex>
      <Show when={user().admin}>
        <button
          class="grey"
          onClick={() => {
            // selected.set("", "users");
            p.close();
          }}
          style={{ "font-size": "1rem", width: "100%" }}
        >
          manage users
        </button>
      </Show>
      <Show when={!user().admin}>
        <Flex justifyContent="center">create server permissions: {user().create_server_permissions.toString()}</Flex>
      </Show>
      <button
        onClick={() => client.logout()}
        class="red"
        style={{ width: "100%" }}
      >
        log out
      </button>
    </Grid>
  );
};

export default Account;
