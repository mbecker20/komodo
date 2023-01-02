import { Route, Routes } from "@solidjs/router";
import { Component, lazy, Show } from "solid-js";
import Topbar from "./components/topbar/Topbar";
import { useUser } from "./state/UserProvider";

const Home = lazy(() => import("./components/home/Home"));
const Deployment = lazy(() => import("./components/deployment/Deployment"));
const Server = lazy(() => import("./components/server/Server"));
const Build = lazy(() => import("./components/build/Build"));
const Users = lazy(() => import("./components/users/Users"));

const App: Component = () => {
  const { user } = useUser();
  return (
    <>
      <Topbar />
      <Routes>
        <Route path="/" component={Home} />
        <Route path="/build/:id" component={Build} />
        <Route path="/deployment/:id" component={Deployment} />
        <Show when={user().admin}>
          <Route path="/users" component={Users} />
        </Show>
      </Routes>
    </>
  );
};

export default App;
