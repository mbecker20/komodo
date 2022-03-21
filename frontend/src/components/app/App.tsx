import { Component, createResource, Match, Switch } from "solid-js";
import { client } from "../..";
import styles from "./App.module.css";
import UserInfo from "../UserInfo";
import { User } from "@monitor/types"
import Login from "../Login";
import { AppStateProvider } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";

const App: Component = () => {
  const { user, logout } = useUser();

  return (
    <div class={styles.App}>
      <Switch>
        <Match when={user()}>
          <AppStateProvider>
            <UserInfo
              user={user() as User}
              logout={logout}
            />
          </AppStateProvider>
        </Match>
        <Match when={user() === undefined}>
          <div>...</div>
        </Match>
        <Match when={user() === false}>
          <Login />
        </Match>
      </Switch>
    </div>
  );
};

export default App;
