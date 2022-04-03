import { Component, For, Show } from "solid-js";
import Icon from "../../../util/icons/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
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
    <Grid class="config-item shadow">
      <Flex alignItems="center" justifyContent="space-between">
        <h1>ports</h1>
        <Flex alignItems="center">
          <Show when={!deployment.ports || deployment.ports.length === 0}>
            <div>none</div>
          </Show>
          <button class="green" onClick={onAdd}>
            <Icon type="plus" />
          </button>
        </Flex>
      </Flex>
      <For each={deployment.ports}>
        {({ local, container }, index) => (
          <Flex justifyContent="space-between" alignItems="center">
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
            <button class="red" onClick={() => onRemove(index())}>
              <Icon type="minus" />
            </button>
          </Flex>
        )}
      </For>
    </Grid>
  );
};

export default Ports;
