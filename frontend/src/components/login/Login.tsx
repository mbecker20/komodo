import { Component } from "solid-js";
import s from "./login.module.css";
import Input from "../util/Input";
import Grid from "../util/layout/Grid";
import { createStore } from "solid-js/store";
import Flex from "../util/layout/Flex";
import { client, pushNotification } from "../..";
import { useUser } from "../../state/UserProvider";
import { combineClasses } from "../../util/helpers";

const Login: Component<{}> = (p) => {
  const [info, set] = createStore({
    username: "",
    password: "",
  });

  const { setUser } = useUser();

  const login = async () => {
    if (info.username.length > 0 && info.password.length > 0) {
      try {
        const user = await client.login(info.username, info.password);
        setUser(() => user);
        pushNotification("good", "logged in!");
      } catch {
        pushNotification("bad", "login failed!");
      }
    } else {
      pushNotification("bad", "provide username or password");
    }
  };

  const signup = async () => {
    if (info.username.length > 0 && info.password.length > 0) {
      try {
        pushNotification("ok", "logging in...");
        const user = await client.signup(info.username, info.password);
        setUser(() => user);
        pushNotification("good", "logged in!");
      } catch {
        pushNotification("bad", "signup failed!");
      }
    } else {
      pushNotification("bad", "provide username or password");
    }
  };

  return (
    <div class={s.Login}>
      <Grid placeItems="center">
        <div class={s.Monitor}>monitor</div>
        <Input
          class={s.LoginItem}
          style={{ width: "20rem" }}
          placeholder="username"
          value={info.username}
          onEdit={(value) => set("username", value)}
        />
        <Input
          class={s.LoginItem}
          style={{ width: "20rem" }}
          type="password"
          placeholder="password"
          value={info.password}
          onEdit={(value) => set("password", value)}
          onConfirm={login}
        />
        <Flex style={{ width: "100%" }} justifyContent="space-between">
          <button class={combineClasses(s.LoginItem, "green")} onClick={login}>
            log in
          </button>
          <button
            class={combineClasses(s.LoginItem, "orange")}
            onClick={signup}
          >
            sign up
          </button>
        </Flex>
        <button
          class={combineClasses(s.LoginItem, "blue")}
          onClick={() => client.loginGithub()}
        >
          log in with github
        </button>
      </Grid>
    </div>
  );
};

export default Login;
