import { Component, createResource, Match, Switch } from "solid-js";
import { client } from "../..";
import styles from "./App.module.css";
import UserInfo from "../UserInfo";
import { User } from "@monitor/types"
import Login from "../Login";

const App: Component = () => {
  const [user, { mutate }] = createResource(() => client.getUser());

  return (
    <div class={styles.App}>
      <Switch>
        <Match when={user()}>
          <UserInfo
            user={user() as User}
            logout={() => {
              client.logout();
              mutate(false);
            }}
          />
        </Match>
        <Match when={user() === undefined}>
          <div>...</div>
        </Match>
        <Match when={user() === false}>
          <Login setUser={(user) => mutate(user as User)} />
        </Match>
      </Switch>
    </div>
  );
};

export default App;
