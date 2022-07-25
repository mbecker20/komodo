import {
  Component,
  createEffect,
  createSignal,
  JSXElement,
} from "solid-js";
import Build from "./components/builds/Build";
import Deployment from "./components/deployment/Deployment";
import Home from "./components/home/Home";
import Server from "./components/server/Server";
// import Sidebar from "./components/sidebar/Sidebar";
import Topbar from "./components/topbar/Topbar";
import Users from "./components/users/Users";
import { useAppState } from "./state/StateProvider";
import { useUser } from "./state/UserProvider";

const App: Component = () => {
  const { selected } = useAppState();
  const { permissions } = useUser();
  const [element, setElement] = createSignal<JSXElement>();
  createEffect(() => {
    if (selected.id()) {
    }
    switch (selected.type()) {
      case "home":
        setElement(
          <div class="content-enter">
            <Home />
          </div>
        );
        return;

      case "deployment":
        setElement(
          <div class="content-enter">
            <Deployment />
          </div>
        );
        break;

      case "build":
        setElement(
          <div class="content-enter">
            <Build />
          </div>
        );
        break;

      case "server":
        setElement(
          <div class="content-enter">
            <Server />
          </div>
        );
        break;

      case "users":
        if (permissions() > 1) {
          setElement(
            <div class="content-enter">
              <Users />
            </div>
          );
        } else {
          setElement(
            <div class="content-enter">
              <Home />
            </div>
          );
        }
        break;
    }
  });
  return (
    <>
      <Topbar />
      {/* <Sidebar /> */}
      {element()}
    </>
  );
};

export default App;
