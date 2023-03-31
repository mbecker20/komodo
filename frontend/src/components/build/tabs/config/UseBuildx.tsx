import { Component, Show } from "solid-js";
import Flex from "../../../shared/layout/Flex";
import { useConfig } from "../Provider";

const UseBuildx: Component<{}> = (p) => {
  const { build, setBuild, userCanUpdate } = useConfig();
  const use_buildx = () => build.docker_build_args?.use_buildx || false;
  return (
    <Flex
      class="config-item shadow"
      alignItems="center"
      justifyContent="space-between"
    >
      <h1>use buildx</h1>
      <Show
        when={userCanUpdate()}
        fallback={<div>{use_buildx() ? "enabled" : "disabled"}</div>}
      >
        <button
          class={use_buildx() ? "green" : "red"}
          onClick={() => setBuild("docker_build_args", "use_buildx", (c) => !c)}
        >
          {use_buildx() ? "enabled" : "disabled"}
        </button>
      </Show>
    </Flex>
  );
};

export default UseBuildx;
