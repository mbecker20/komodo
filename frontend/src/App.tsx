import { Component, Match, Switch } from "solid-js";
import Deployment from "./components/deployment/Deployment";
import Sidebar from "./components/sidebar/Sidebar";
import Topbar from "./components/topbar/Topbar";
import { useAppState } from "./state/StateProvider";

const App: Component = () => {
  const { selected } = useAppState();
  return (
    <>
      <Topbar />
      <Sidebar />
      <Switch>
        <Match when={selected.type() === "deployment"}>
          <Deployment id={selected.id()} />
        </Match>
        {/* <Match when={selected.type() === "server"}></Match> */}
        {/* <Match when={selected.type() === "build"}></Match> */}
      </Switch>
    </>
  );
};

export default App;
