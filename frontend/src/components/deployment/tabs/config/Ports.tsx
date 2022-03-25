import { Deployment } from "@monitor/types";
import { Component, For, Show } from "solid-js";
import { DeepReadonly, SetStoreFunction } from "solid-js/store";
import { combineClasses } from "../../../../util/helpers";
import Icon from "../../../util/icons/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import s from "../../deployment.module.css";

const Ports: Component<{
  deployment: DeepReadonly<Deployment>;
  setDeployment: SetStoreFunction<Deployment>;
}> = (p) => {
  return (
    <Grid class={combineClasses(s.ConfigItem, "shadow")}>
      <Flex alignItems="center">
        <div class={s.ItemHeader}>ports</div>
        <Show when={!p.deployment.ports || p.deployment.ports.length === 0}>
          <div>none</div>
        </Show>
        <button>
          <Icon type="plus" />
        </button>
      </Flex>
      <For each={p.deployment.ports}>
        {({ local, container }) => (
          <Flex justifyContent="center">
            <Input
              placeholder="system"
              value={local}
              style={{ width: "40%" }}
            />
            {" : "}
            <Input
              placeholder="container"
              value={container}
              style={{ width: "40%" }}
            />
            <button>
              <Icon type="minus" />
            </button>
          </Flex>
        )}
      </For>
    </Grid>
  );
};

export default Ports;
