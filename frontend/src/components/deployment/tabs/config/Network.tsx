import { Deployment } from "@monitor/types";
import { Component, For } from "solid-js";
import { DeepReadonly, SetStoreFunction } from "solid-js/store";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import s from "../../deployment.module.css";

const Network: Component<{
  deployment: DeepReadonly<Deployment>;
  setDeployment: SetStoreFunction<Deployment>;
}> = (p) => {
  return (
    <Flex class={combineClasses(s.ConfigItem, "shadow")} justifyContent="space-between">
      <div class={s.ItemHeader}>network</div>
      <Input value={p.deployment.network} />
    </Flex>
  );
};

export default Network;
