import { Deployment } from "@monitor/types";
import { Component } from "solid-js";
import { DeepReadonly, SetStoreFunction } from "solid-js/store";
import { useToggle } from "../../../../util/hooks";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Menu from "../../../util/menu/Menu";
import s from "../../deployment.module.css";

const Image: Component<{
  deployment: DeepReadonly<Deployment>;
  setDeployment: SetStoreFunction<Deployment>;
}> = (p) => {
  const [show, toggle] = useToggle();
  return (
    <Grid>
      <div class={s.ItemHeader}>image</div>
      <Flex>
        <div>select build: </div>
        <Menu
          show={show()}
          target={<div onClick={toggle}>build1</div>}
          content={<div></div>}
          position="bottom center"
        />
      </Flex>
    </Grid>
  );
};

export default Image;
