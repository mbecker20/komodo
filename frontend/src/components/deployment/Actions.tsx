import { Deployment } from "@monitor/types";
import { Component } from "solid-js";
import Icon from "../util/icons/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import s from "./deployment.module.css";

const Actions: Component<{ deployment: Deployment }> = (p) => {
  return (
    <Grid class={s.Actions}>
      <div class={s.Header}>actions</div>
      <Flex alignItems="center" class={s.Action}>
        deploy:{" "}
        <button>
          <Icon type="arrow-down" />
        </button>
      </Flex>
      <Flex alignItems="center" class={s.Action}>
        redeploy:{" "}
        <button>
          <Icon type="arrow-down" />
        </button>
      </Flex>
      <Flex alignItems="center" class={s.Action}>
        start:{" "}
        <button>
          <Icon type="arrow-down" />
        </button>
      </Flex>
      <Flex alignItems="center" class={s.Action}>
        stop:{" "}
        <button>
          <Icon type="arrow-down" />
        </button>
      </Flex>
    </Grid>
  );
};

export default Actions;
