import { Component, Match, Switch } from "solid-js";
import Build from "./components/builds/Build";
import Deployment from "./components/deployment/Deployment";
import Server from "./components/server/Server";
import Sidebar from "./components/sidebar/Sidebar";
import Topbar from "./components/topbar/Topbar";
import Users from "./components/users/Users";
import { useAppState } from "./state/StateProvider";
import { useUser } from "./state/UserProvider";

const App: Component = () => {
  const { selected } = useAppState();
  const { permissions } = useUser();
  return (
    <>
      <Topbar />
      <Sidebar />
      <Switch>
        <Match when={selected.type() === "deployment"}>
          <Deployment />
        </Match>
        <Match when={selected.type() === "server"}>
          <Server />
        </Match>
        <Match when={selected.type() === "build"}>
          <Build />
        </Match>
        <Match when={selected.type() === "users" && permissions() > 1}>
          <Users />
        </Match>
      </Switch>
    </>
  );
};

export default App;
