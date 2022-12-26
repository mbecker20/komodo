import { Route, Routes } from "@solidjs/router";
import type { Component } from "solid-js";
import Build from "./components/build/Build";
import Deployment from "./components/deployment/Deployment";
import Home from "./components/home/Home";
import Server from "./components/server/Server";
import Topbar from "./components/topbar/Topbar";

const App: Component = () => {
  return (
    <>
      <Topbar />
      <Routes>
        <Route path="/" component={Home} />
        <Route path="/build/:id" component={Build} />
        <Route path="/deployment/:id" component={Deployment} />
        <Route path="/server/:id" component={Server} />
      </Routes>
    </>
  );
};

export default App;
