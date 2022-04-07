import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { readablePermissions } from "../../util/helpers";
import ConfirmButton from "../util/ConfirmButton";
import Grid from "../util/layout/Grid";
import s from "./topbar.module.scss";

const Account: Component<{ close: () => void }> = (p) => {
  const { logout, selected } = useAppState();
  const { username, permissions } = useUser();
  return (
    <Grid class={s.Account} placeItems="center end">
      <div>{username()}</div>
      <div>permissions: {readablePermissions(permissions())}</div>
      <Show when={permissions() > 1}>
        <button
          class="blue"
          onClick={() => {
            selected.set("", "users");
            p.close();
          }}
          style={{ "font-size": "1rem" }}
        >
          manage users
        </button>
      </Show>
      <ConfirmButton onConfirm={logout} color="red">
        log out
      </ConfirmButton>
    </Grid>
  );
};

export default Account;
