import { Component } from "solid-js";
import Sidebar from "./components/sidebar/Sidebar";
import Topbar from "./components/topbar/Topbar";

const App: Component = () => {
  return (
    <>
      <Topbar />
      <Sidebar />
    </>
  );
};

export default App;
