import { Deployment } from "@monitor/types";
import { Component } from "solid-js";
import { DeepReadonly, SetStoreFunction } from "solid-js/store";
import Grid from "../../../util/layout/Grid";
import s from "../../deployment.module.css";

const Network: Component<{
  deployment: DeepReadonly<Deployment>;
  setDeployment: SetStoreFunction<Deployment>;
}> = (p) => {
  return (
    <Grid class={s.ConfigItem}>
      <div class={s.ItemHeader}>network</div>
    </Grid>
  );
};

export default Network;
