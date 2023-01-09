import { Component, createSignal, Show } from "solid-js";
import { version_to_string } from "../../../../util/helpers";
import Input from "../../../shared/Input";
import Flex from "../../../shared/layout/Flex";
import { useConfig } from "../Provider";

const Version: Component<{}> = (p) => {
  const { build, setBuild, userCanUpdate } = useConfig();
  return (
    <Flex
      class="config-item shadow"
      alignItems="center"
      justifyContent="space-between"
    >
      <h1>version</h1>
      <Flex alignItems="center">
        <h1>v{version_to_string(build.version)}</h1>
        <Show when={userCanUpdate()}>
          <button
            class="blue"
            onClick={() => {
              setBuild("version", { major: build.version.major + 1, patch: 0 });
            }}
          >
            major +
          </button>
          <button
            class="blue"
            onClick={() => {
              setBuild("version", { minor: build.version.minor + 1, patch: 0 });
            }}
          >
            minor +
          </button>
        </Show>
      </Flex>
    </Flex>
  );
};

export default Version;
