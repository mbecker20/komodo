import { Component, For, Show } from "solid-js";
import Icon from "../../../util/icons/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import { useConfig } from "./Provider";

const Volumes: Component<{}> = (p) => {
  const { deployment, setDeployment } = useConfig();
  const onAdd = () => {
    setDeployment("volumes", (volumes: any) => [
      ...volumes,
      { local: "", container: "" },
    ]);
  };
  const onRemove = (index: number) => {
    setDeployment("volumes", (volumes) => volumes!.filter((_, i) => i !== index));
  };
  return (
    <Grid class="config-item shadow">
      <Flex alignItems="center">
        <h1>volumes</h1>
        <Show when={!deployment.volumes || deployment.volumes.length === 0}>
          <div>none</div>
        </Show>
        <button class="green" onClick={onAdd}>
          <Icon type="plus" />
        </button>
      </Flex>
      <For each={deployment.volumes}>
        {({ local, container }, index) => (
          <Flex justifyContent="center">
            <Input
              placeholder="system"
              value={local}
              style={{ width: "40%" }}
              onEdit={(value) =>
                setDeployment("volumes", index(), "local", value)
              }
            />
            {" : "}
            <Input
              placeholder="container"
              value={container}
              style={{ width: "40%" }}
              onEdit={(value) =>
                setDeployment("volumes", index(), "container", value)
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

export default Volumes;
