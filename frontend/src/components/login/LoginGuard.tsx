import { Component, Match, Switch } from "solid-js";
import {
  LOGGED_IN_DISABLED,
  LOGGED_IN_ENABLED,
  SIGNED_OUT,
  UNKNOWN,
  useUser,
} from "../../state/UserProvider";
import Loading from "../util/loading/Loading";
import Login from "./Login";
import s from "./login.module.scss";
import NotActivated from "./NotActivated";

const LoginGuard: Component = (p) => {
  const { loginStatus } = useUser();
  return (
    <Switch>
      <Match when={loginStatus() === LOGGED_IN_ENABLED}>{p.children}</Match>
      <Match when={loginStatus() === LOGGED_IN_DISABLED}>
        <NotActivated />
      </Match>
      <Match when={loginStatus() === SIGNED_OUT}>
        <Login />
      </Match>
      <Match when={loginStatus() === UNKNOWN}>
        <div class={s.Login}>
          <Loading type="three-dot" scale={1} />
        </div>
      </Match>
    </Switch>
  );
};

export default LoginGuard;
