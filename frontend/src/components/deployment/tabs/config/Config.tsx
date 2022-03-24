import { Component } from "solid-js";
import s from "../../deployment.module.css";
import { Deployment } from "@monitor/types";
import { createStore } from "solid-js/store";
import Grid from "../../../util/layout/Grid";
import Image from "./Image";

const Config: Component<{ deployment: Deployment }> = (p) => {
  const [deployment, setDeployment] = createStore(p.deployment);

  return (
    <Grid class={s.Config}>
      <Image deployment={deployment} setDeployment={setDeployment} />
    </Grid>
  );
};

export default Config;
