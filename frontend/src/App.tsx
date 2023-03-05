import { Route, Routes } from "@solidjs/router";
import { Component, lazy, Show } from "solid-js";
import Topbar from "./components/topbar/Topbar";
import { useUser } from "./state/UserProvider";

const Home = lazy(() => import("./components/home/Home"));
const Deployment = lazy(() => import("./components/deployment/Deployment"));
const Server = lazy(() => import("./components/server/Server"));
const Build = lazy(() => import("./components/build/Build"));
const Users = lazy(() => import("./components/users/Users"));
const User = lazy(() => import("./components/users/User"));
const Stats = lazy(() => import("./components/stats/Stats"));
const Account = lazy(() => import("./components/Account"));

const App: Component = () => {
  const { user } = useUser();
  return (
    <div class="app">
      <Topbar />
      <Routes>
        <Route path="/" component={Home} />
        <Route path="/build/:id" component={Build} />
        <Route path="/deployment/:id" component={Deployment} />
        <Route path="/server/:id" component={Server} />
        <Route path="/server/:id/stats" component={Stats} />
        <Route path="/account" component={Account} />
        <Show when={user().admin}>
          <Route path="/users" component={Users} />
          <Route path="/user/:id" component={User} />
        </Show>
      </Routes>
    </div>
  );
};

export default App;
