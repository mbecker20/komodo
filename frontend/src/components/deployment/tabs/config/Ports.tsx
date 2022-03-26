import { Component, For, Show } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Icon from "../../../util/icons/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import s from "../../deployment.module.css";
import { useConfig } from "./Provider";

const Ports: Component<{}> = (p) => {
  const { deployment, setDeployment } = useConfig();
  const onAdd = () => {
    setDeployment("ports", (ports: any) => [
      ...ports,
      { local: "", container: "" },
    ]);
  };
  const onRemove = (index: number) => {
    setDeployment("ports", (ports) => ports!.filter((_, i) => i !== index));
  };
  return (
    <Grid class={combineClasses(s.ConfigItem, "shadow")}>
      <Flex alignItems="center">
        <div class={s.ItemHeader}>ports</div>
        <Show when={!deployment.ports || deployment.ports.length === 0}>
          <div>none</div>
        </Show>
        <button onClick={onAdd}>
          <Icon type="plus" />
        </button>
      </Flex>
      <For each={deployment.ports}>
        {({ local, container }, index) => (
          <Flex justifyContent="center">
            <Input
              placeholder="system"
              value={local}
              style={{ width: "40%" }}
              onConfirm={(value) =>
                setDeployment("ports", index(), "local", value)
              }
            />
            {" : "}
            <Input
              placeholder="container"
              value={container}
              style={{ width: "40%" }}
              onConfirm={(value) =>
                setDeployment("ports", index(), "container", value)
              }
            />
            <button onClick={() => onRemove(index())}>
              <Icon type="minus" />
            </button>
          </Flex>
        )}
      </For>
    </Grid>
  );
};

export default Ports;
