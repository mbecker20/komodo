import { Component, For, Show } from "solid-js";
import Icon from "../../../shared/Icon";
import Input from "../../../shared/Input";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import { useConfig } from "../Provider";

const ExtraArgs: Component<{}> = (p) => {
  const { build, setBuild, userCanUpdate } = useConfig();
  const onAdd = () => {
    setBuild("docker_build_args", "extra_args", (extra_args: any) => [
      ...extra_args,
      "",
    ]);
  };
  const onRemove = (index: number) => {
    setBuild("docker_build_args", "extra_args", (extra_args) =>
      extra_args!.filter((_, i) => i !== index)
    );
  };
  return (
    <Grid class="config-item shadow">
      <Flex justifyContent="space-between" alignItems="center">
        <h1>extra args</h1>
        <Show when={userCanUpdate()}>
          <button class="green" onClick={onAdd}>
            <Icon type="plus" />
          </button>
        </Show>
      </Flex>
      <For each={[...build.docker_build_args!.extra_args!.keys()]}>
        {(_, index) => (
          <Flex
            justifyContent={userCanUpdate() ? "space-between" : undefined}
            alignItems="center"
            style={{ "flex-wrap": "wrap" }}
          >
            <Input
              placeholder="--extra-arg=value"
              value={build.docker_build_args!.extra_args![index()]}
              style={{ width: "80%" }}
              onEdit={(value) =>
                setBuild("docker_build_args", "extra_args", index(), value)
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

export default ExtraArgs;
