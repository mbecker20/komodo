import { Deployment } from "@monitor/types";
import { Component, For, Show } from "solid-js";
import { DeepReadonly, SetStoreFunction } from "solid-js/store";
import { combineClasses } from "../../../../util/helpers";
import Icon from "../../../util/icons/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import s from "../../deployment.module.css";

const Env: Component<{
  deployment: DeepReadonly<Deployment>;
  setDeployment: SetStoreFunction<Deployment>;
}> = (p) => {
  return (
    <Grid class={combineClasses(s.ConfigItem, "shadow")}>
      <Flex alignItems="center">
        <div class={s.ItemHeader}>environment</div>
        <Show
          when={
            !p.deployment.environment || p.deployment.environment.length === 0
          }
        >
          <div>none</div>
        </Show>
        <button>
          <Icon type="plus" />
        </button>
      </Flex>
      <For each={p.deployment.environment}>
        {({ variable, value }) => (
          <Flex justifyContent="center">
            <Input
              placeholder="variable"
              value={variable}
              style={{ width: "40%" }}
            />
            {" : "}
            <Input placeholder="value" value={value} style={{ width: "40%" }} />
            <button>
              <Icon type="minus" />
            </button>
          </Flex>
        )}
      </For>
    </Grid>
  );
};

export default Env;
