import { Component, For } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import s from "../../deployment.module.css";
import { useConfig } from "./Provider";

const Network: Component<{}> = (p) => {
  const { deployment, setDeployment } = useConfig();
  return (
    <Flex
      class={combineClasses(s.ConfigItem, "shadow")}
      justifyContent="space-between"
    >
      <div class={s.ItemHeader}>network</div>
      <Input value={deployment.network} onConfirm={(value) => setDeployment("network", value)} />
    </Flex>
  );
};

export default Network;
