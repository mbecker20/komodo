import { Deployment } from "@monitor/types";
import { Component } from "solid-js";
import { DeepReadonly, SetStoreFunction } from "solid-js/store";
import Grid from "../../../util/layout/Grid";
import s from "../../deployment.module.css";

const Image: Component<{
  deployment: DeepReadonly<Deployment>;
  setDeployment: SetStoreFunction<Deployment>;
}> = (p) => {
  return (
    <Grid>
      <div class={s.ItemHeader}>image</div>
    </Grid>
  );
};

export default Image;
