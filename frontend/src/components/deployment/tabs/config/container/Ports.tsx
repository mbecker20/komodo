import { Component, For, Show } from "solid-js";
import { combineClasses } from "../../../../../util/helpers";
import Icon from "../../../../shared/Icon";
import Input from "../../../../shared/Input";
import Flex from "../../../../shared/layout/Flex";
import Grid from "../../../../shared/layout/Grid";
import { useConfig } from "../Provider";

const Ports: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const onAdd = () => {
    setDeployment("docker_run_args", "ports", (ports: any) => [
      ...ports,
      { local: "", container: "" },
    ]);
  };
  const onRemove = (index: number) => {
    setDeployment("docker_run_args", "ports", (ports) => ports!.filter((_, i) => i !== index));
  };
  return (
    <Show when={deployment.docker_run_args.network !== "host"}>
      <Grid class={combineClasses("config-item shadow")}>
        <Flex alignItems="center" justifyContent="space-between">
          <h1>ports</h1>
          <Flex alignItems="center">
            <Show
              when={
                !deployment.docker_run_args.ports ||
                deployment.docker_run_args.ports.length === 0
              }
            >
              <div>none</div>
            </Show>
            <Show when={userCanUpdate()}>
              <button class="green" onClick={onAdd}>
                <Icon type="plus" />
              </button>
            </Show>
          </Flex>
        </Flex>
        <For each={deployment.docker_run_args.ports}>
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
                    "ports",
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
                    "ports",
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
    </Show>
  );
};

export default Ports;
