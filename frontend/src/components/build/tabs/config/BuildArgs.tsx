import { useParams } from "@solidjs/router";
import {
  Component,
  createEffect,
  createResource,
  createSignal,
  For,
  Show,
} from "solid-js";
import { client } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { ServerStatus } from "../../../../types";
import {
  parseDotEnvToEnvVars,
  parseEnvVarseToDotEnv,
} from "../../../../util/helpers";
import { useToggle } from "../../../../util/hooks";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
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
  const { aws_builder_config, builds, serverSecrets } = useAppState();
  const [show, toggle] = useToggle();
  const [buildArgs, setBuildArgs] = createSignal("");
  const params = useParams();
  const { build, setBuild, server } = useConfig();
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
  const secrets = () => {
    if (builds.get(params.id)?.server_id) {
      return (
        serverSecrets.get(
          builds.get(params.id)!.server_id!,
          server()?.status || ServerStatus.NotOk
        ) || []
      );
    } else if (build.aws_config) {
      const ami_name =
        build.aws_config?.ami_name || aws_builder_config()?.default_ami_name;
      return ami_name
        ? aws_builder_config()?.available_ami_accounts![ami_name].secrets || []
        : [];
    } else return [];
  };
  let ref: HTMLTextAreaElement;
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
        <Grid>
          <Show when={secrets()?.length || 0 > 0}>
            <Flex class="wrap" justifyContent="flex-end" alignItems="center">
              <h2 class="dimmed">secrets:</h2>
              <For each={secrets()}>
                {(secret) => (
                  <button
                    class="blue"
                    onClick={() =>
                      setBuildArgs(
                        (args) =>
                          args.slice(0, ref.selectionStart) +
                          `[[${secret}]]` +
                          args.slice(ref.selectionStart, undefined)
                      )
                    }
                  >
                    {secret}
                  </button>
                )}
              </For>
            </Flex>
          </Show>
          <TextArea
            ref={ref! as any}
            class="scroller"
            placeholder="VARIABLE=value   #example"
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
        </Grid>
      )}
    />
  );
};

export default BuildArgs;
