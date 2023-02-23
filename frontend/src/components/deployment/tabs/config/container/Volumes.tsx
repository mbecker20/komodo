import { Component, For, Show } from "solid-js";
import { combineClasses } from "../../../../../util/helpers";
import Icon from "../../../../shared/Icon";
import Input from "../../../../shared/Input";
import Flex from "../../../../shared/layout/Flex";
import Grid from "../../../../shared/layout/Grid";
import { useConfig } from "../Provider";

const Volumes: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const onAdd = () => {
    setDeployment("docker_run_args", "volumes", (volumes: any) => [
      ...volumes,
      { local: "", container: "" },
    ]);
  };
  const onRemove = (index: number) => {
    setDeployment("docker_run_args", "volumes", (volumes) =>
      volumes!.filter((_, i) => i !== index)
    );
  };
  return (
    <Grid class={combineClasses("config-item shadow")}>
      <Flex justifyContent="space-between" alignItems="center">
        <h1>volumes</h1>
        <Show when={userCanUpdate()}>
          <button class="green" onClick={onAdd}>
            <Icon type="plus" />
          </button>
        </Show>
      </Flex>
      <For each={deployment.docker_run_args.volumes}>
        {({ local, container }, index) => (
          <Flex
            justifyContent={userCanUpdate() ? "space-between" : undefined}
            alignItems="center"
            style={{ "flex-wrap": "wrap" }}
          >
            <Input
              placeholder="system"
              value={local}
              style={{ width: "40%" }}
              onEdit={(value) =>
                setDeployment(
                  "docker_run_args",
                  "volumes",
                  index(),
                  "local",
                  value
                )
              }
              disabled={!userCanUpdate()}
            />
            {" : "}
            <Input
              placeholder="container"
              value={container}
              style={{ width: "40%" }}
              onEdit={(value) =>
                setDeployment(
                  "docker_run_args",
                  "volumes",
                  index(),
                  "container",
                  value
                )
              }
              disabled={!userCanUpdate()}
            />
            <Show when={userCanUpdate()}>
              <button class="red" onClick={() => onRemove(index())}>
                <Icon type="minus" />
              </button>
            </Show>
          </Flex>
        )}
      </For>
    </Grid>
  );
};

export default Volumes;
