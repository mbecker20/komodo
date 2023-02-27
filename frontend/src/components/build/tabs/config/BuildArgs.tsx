import { Component, createEffect, createSignal, Show } from "solid-js";
import {
  parseDotEnvToEnvVars,
  parseEnvVarseToDotEnv,
} from "../../../../util/helpers";
import { useToggle } from "../../../../util/hooks";
import Flex from "../../../shared/layout/Flex";
import CenterMenu from "../../../shared/menu/CenterMenu";
import TextArea from "../../../shared/TextArea";
import { useConfig } from "../Provider";

const BuildArgs: Component<{}> = (p) => {
  const { build, userCanUpdate } = useConfig();
  return (
    <Flex
      class="config-item shadow"
      alignItems="center"
      justifyContent="space-between"
    >
      <h1>build args</h1>
      <Flex alignItems="center">
        <Show
          when={
            !build.docker_build_args?.build_args ||
            build.docker_build_args.build_args.length === 0
          }
        >
          <div>none</div>
        </Show>
        <Show when={userCanUpdate()}>
          <EditBuildArgs />
        </Show>
      </Flex>
    </Flex>
  );
};

const EditBuildArgs: Component<{}> = (p) => {
  const [show, toggle] = useToggle();
  const [buildArgs, setBuildArgs] = createSignal("");
  const { build, setBuild } = useConfig();
  createEffect(() => {
    setBuildArgs(
      parseEnvVarseToDotEnv(
        build.docker_build_args?.build_args
          ? build.docker_build_args.build_args
          : []
      )
    );
  });
  const toggleShow = () => {
    if (show()) {
      setBuild("docker_build_args", {
        build_args: parseDotEnvToEnvVars(buildArgs()),
      });
    }
    toggle();
  };
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      title={`${build.name} build args`}
      target="edit"
      targetClass="blue"
      leftOfX={() => (
        <button class="green" onClick={toggleShow}>
          confirm
        </button>
      )}
      content={() => (
        <TextArea
          class="scroller"
          value={buildArgs()}
          onEdit={setBuildArgs}
          style={{
            width: "1000px",
            "max-width": "90vw",
            height: "80vh",
            padding: "1rem",
          }}
          spellcheck={false}
        />
      )}
    />
  );
};

export default BuildArgs;
