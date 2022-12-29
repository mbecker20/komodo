import { Component } from "solid-js";
import s from "./login.module.scss";
import Input from "../shared/Input";
import Grid from "../shared/layout/Grid";
import { createStore } from "solid-js/store";
import Flex from "../shared/layout/Flex";
import { client, pushNotification, URL } from "../..";
import { combineClasses } from "../../util/helpers";
import Icon from "../shared/Icon";
import { useUser } from "../../state/UserProvider";

const Login: Component<{}> = (p) => {
  const [info, set] = createStore({
    username: "",
    password: "",
  });

  const { setUser } = useUser();

  const login = async () => {
    if (info.username.length > 0 && info.password.length > 0) {
      try {
        const user = await client.login(info);
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
        const user = await client.signup(info);
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
          onEnter={login}
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
          onClick={() => location.replace(`${URL}/auth/github/login`)}
        >
          log in with github <Icon type="github" />
        </button>
        <button
          class={combineClasses(s.LoginItem, "red")}
          onClick={() => location.replace(`${URL}/auth/google/login`)}
        >
          log in with google <Icon type="google" />
        </button>
      </Grid>
    </div>
  );
};

export default Login;
