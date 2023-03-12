import {
  Component,
  createEffect,
  createResource,
  createSignal,
  For,
  Show,
} from "solid-js";
import { client } from "../../../../..";
import { useAppState } from "../../../../../state/StateProvider";
import { ServerStatus } from "../../../../../types";
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
        <Flex alignItems="center">
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
  const { serverSecrets } = useAppState();
  const [show, toggle] = useToggle();
  const [dotenv, setDotEnv] = createSignal("");
  const { deployment, setDeployment, server } = useConfig();
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
  const secrets = () =>
    serverSecrets.get(
      deployment.server_id,
      server()?.status || ServerStatus.NotOk
    ) || [];
  let ref: HTMLTextAreaElement;
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      title={`${deployment.name} environment`}
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
                      setDotEnv(
                        (env) =>
                          env.slice(0, ref.selectionStart) +
                          `[[${secret}]]` +
                          env.slice(ref.selectionStart, undefined)
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
            value={dotenv()}
            onEdit={setDotEnv}
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

export default Env;
