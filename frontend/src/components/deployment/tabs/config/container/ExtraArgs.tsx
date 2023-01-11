import { Component, For, Show } from "solid-js";
import Icon from "../../../../shared/Icon";
import Input from "../../../../shared/Input";
import Flex from "../../../../shared/layout/Flex";
import Grid from "../../../../shared/layout/Grid";
import { useConfig } from "../Provider";

const ExtraArgs: Component<{}> = (p) => {
	const { deployment, setDeployment, userCanUpdate } = useConfig();
	const onAdd = () => {
    setDeployment("docker_run_args", "extra_args", (extra_args: any) => [
      ...extra_args,
      "",
    ]);
  };
  const onRemove = (index: number) => {
    setDeployment("docker_run_args", "extra_args", (extra_args) =>
      extra_args!.filter((_, i) => i !== index)
    );
  };
	return (
    <Grid class="config-item shadow">
      <Flex justifyContent="space-between" alignItems="center">
        <h1>extra args</h1>
        <Flex alignItems="center">
          <Show
            when={
              !deployment.docker_run_args.extra_args ||
              deployment.docker_run_args.extra_args.length === 0
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
      <For each={[...deployment.docker_run_args.extra_args!.keys()]}>
        {(_, index) => (
          <Flex
            justifyContent={userCanUpdate() ? "space-between" : undefined}
            alignItems="center"
            style={{ "flex-wrap": "wrap" }}
          >
            <Input
              placeholder="--extra-arg=value"
              value={deployment.docker_run_args.extra_args![index()]}
              style={{ width: "80%" }}
              onEdit={(value) =>
                setDeployment("docker_run_args", "extra_args", index(), value)
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
}

export default ExtraArgs;