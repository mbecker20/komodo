import { Component, createEffect, createSignal, Show } from "solid-js";
import {
  combineClasses,
  parseDotEnvToEnvVars,
  parseEnvVarseToDotEnv,
} from "../../../../../util/helpers";
import { useToggle } from "../../../../../util/hooks";
import Flex from "../../../../shared/layout/Flex";
import Grid from "../../../../shared/layout/Grid";
import CenterMenu from "../../../../shared/menu/CenterMenu";
import TextArea from "../../../../shared/TextArea";
import { useConfig } from "../Provider";

const Env: Component<{}> = (p) => {
  const { deployment, userCanUpdate } = useConfig();
  return (
    <Grid class={combineClasses("config-item shadow")}>
      <Flex alignItems="center" justifyContent="space-between">
        <h1>environment</h1>
        <Flex alignItems="center" gap="0.2rem">
          <Show
            when={
              !deployment.docker_run_args.environment ||
              deployment.docker_run_args.environment.length === 0
            }
          >
            <div>none</div>
          </Show>
          <Show when={userCanUpdate()}>
            <EditDotEnv />
          </Show>
        </Flex>
      </Flex>
    </Grid>
  );
};

const EditDotEnv: Component<{}> = (p) => {
  const [show, toggle] = useToggle();
  const [dotenv, setDotEnv] = createSignal("");
  const { deployment, setDeployment } = useConfig();
  createEffect(() => {
    setDotEnv(
      parseEnvVarseToDotEnv(
        deployment.docker_run_args.environment
          ? deployment.docker_run_args.environment
          : []
      )
    );
  });
  const toggleShow = () => {
    if (show()) {
      setDeployment("docker_run_args", {
        environment: parseDotEnvToEnvVars(dotenv()),
      });
    }
    toggle();
  };
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      title={`${deployment.name} environment`}
      target="edit"
      targetClass="blue"
      leftOfX={
        () => <button class="green" onClick={toggleShow}>
          confirm
        </button>
      }
      content={() => (
        <TextArea
          class="scroller"
          value={dotenv()}
          onEdit={setDotEnv}
          style={{ width: "40rem", "max-width": "90vw", height: "80vh" }}
          spellcheck={false}
        />
      )}
    />
  );
};

export default Env;
