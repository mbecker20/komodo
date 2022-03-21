import { Component } from "solid-js";
import Input from "./util/Input";
import Grid from "./util/layout/Grid";
import { createStore } from "solid-js/store";
import Flex from "./util/layout/Flex";
import { client, pushNotification } from "..";
import { User } from "@monitor/types";

const Login: Component<{ setUser: (user: User | false) => void }> = (p) => {
  const [info, set] = createStore({
    username: "",
    password: "",
  });

  const login = async () => {
    if (info.username.length > 0 && info.password.length > 0) {
      try {
        const user = await client.login(info.username, info.password);
        p.setUser(user);
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
        p.setUser(user);
        pushNotification("good", "logged in!");
      } catch {
        pushNotification("bad", "signup failed!");
      }
    } else {
      pushNotification("bad", "provide username or password");
    }
  };

  return (
    <Grid>
      <Input
        placeholder="username"
        value={info.username}
        onEdit={(value) => set("username", value)}
      />
      <Input
        type="password"
        placeholder="password"
        value={info.password}
        onEdit={(value) => set("password", value)}
      />
      <Flex style={{ width: "100%" }} justifyContent="space-between">
        <button onClick={login}>log in</button>
        <button onClick={signup}>sign up</button>
      </Flex>
      <button onClick={() => client.loginGithub()}>log in with github</button>
    </Grid>
  );
};

export default Login;
