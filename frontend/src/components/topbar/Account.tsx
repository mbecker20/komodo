import { A } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import s from "./topbar.module.scss";

const Account: Component<{ close: () => void }> = (p) => {
  const { user } = useUser();
  const { isMobile } = useAppDimensions();
  const { logout } = useAppState();
  return (
    <Grid gap="0.5rem" class={s.Account} placeItems="center end">
      <Show when={isMobile()}>
        <Flex justifyContent="center">{user().username}</Flex>
      </Show>
      <Flex justifyContent="center">admin: {user().admin.toString()}</Flex>
      <Show when={user().admin}>
        <A
          href="/users"
          class="grey"
          onClick={() => p.close()}
          style={{ "font-size": "1rem", width: "100%" }}
        >
          manage users
        </A>
      </Show>
      <Show when={!user().admin}>
        <Flex justifyContent="center">
          create server permissions:{" "}
          {user().create_server_permissions.toString()}
        </Flex>
      </Show>
      <A
        href="/account"
        class="grey"
        onClick={() => p.close()}
        style={{ "font-size": "1rem", width: "100%" }}
      >
        account
      </A>
      <button onClick={() => logout()} class="red" style={{ width: "100%" }}>
        log out
      </button>
    </Grid>
  );
};

export default Account;
