import { Component } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { readablePermissions } from "../../util/helpers";
import ConfirmButton from "../util/ConfirmButton";
import Grid from "../util/layout/Grid";
import s from "./topbar.module.css";

const Account: Component<{}> = (p) => {
  const { logout } = useAppState();
  const { user, username } = useUser();
  return (
    <Grid class={s.Account} placeItems="center end">
      <div>{username()}</div>
      <div>permissions: {readablePermissions(user()!.permissions!)}</div>
      <ConfirmButton onConfirm={logout} color="red">
        log out
      </ConfirmButton>
    </Grid>
  );
};

export default Account;
