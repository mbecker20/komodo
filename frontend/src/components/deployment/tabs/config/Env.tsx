import { Deployment } from "@monitor/types";
import { Component } from "solid-js";
import { DeepReadonly, SetStoreFunction } from "solid-js/store";
import Grid from "../../../util/layout/Grid";
import s from "../../deployment.module.css";

const Env: Component<{
  deployment: DeepReadonly<Deployment>;
  setDeployment: SetStoreFunction<Deployment>;
}> = (p) => {
  return (
    <Grid class={s.ConfigItem}>
      <div class={s.ItemHeader}>environment</div>
    </Grid>
  );
};

export default Env;
