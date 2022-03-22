import { Component, Match, Switch } from "solid-js";
import {
  LOGGED_IN,
  SIGNED_OUT,
  UNKNOWN,
  useUser,
} from "../../state/UserProvider";
import Login from "./Login";

const LoginGuard: Component = (p) => {
  const { loginStatus } = useUser();
  return (
    <Switch>
      <Match when={loginStatus() === LOGGED_IN}>{p.children}</Match>
      <Match when={loginStatus() === SIGNED_OUT}>
        <Login />
      </Match>
      <Match when={loginStatus() === UNKNOWN}>
        <div>...</div> {/* Loading */}
      </Match>
    </Switch>
  );
};

export default LoginGuard;
