import { Component } from "solid-js";
import s from "./home.module.scss"
import Summary from "./Summary";
import Updates from "./Updates/Updates";

const Home: Component<{}> = (p) => {
	return (
    <>
      <div>
        <Summary />
      </div>
      <div>
        <Updates />
      </div>
    </>
  );
}

export default Home;