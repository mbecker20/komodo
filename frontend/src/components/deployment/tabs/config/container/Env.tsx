import { EnvironmentVar } from "@monitor/types";
import { parseDotEnvToEnvVars, parseEnvVarseToDotEnv } from "@monitor/util";
import { Component, createEffect, createSignal, Show } from "solid-js";
import { useTheme } from "../../../../../state/ThemeProvider";
import { combineClasses } from "../../../../../util/helpers";
import { useToggle } from "../../../../../util/hooks";
import Button from "../../../../util/Button";
import Flex from "../../../../util/layout/Flex";
import Grid from "../../../../util/layout/Grid";
import CenterMenu from "../../../../util/menu/CenterMenu";
import TextArea from "../../../../util/TextArea";
import { useConfig } from "../Provider";

const Env: Component<{}> = (p) => {
  const { deployment, userCanUpdate } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Grid class={combineClasses("config-item shadow", themeClass())}>
      <Flex alignItems="center" justifyContent="space-between">
        <h1>environment</h1>
        <Flex alignItems="center" gap="0.2rem">
          <Show
            when={
              !deployment.environment || deployment.environment.length === 0
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
        deployment.environment
          ? (deployment.environment as EnvironmentVar[])
          : []
      )
    );
  });
  const toggleShow = () => {
    if (show()) {
      setDeployment("environment", parseDotEnvToEnvVars(dotenv()));
    }
    toggle()
  }
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      title={`${deployment.name} environment`}
      target="edit"
      targetClass="blue"
      leftOfX={
        <Button class="green" onClick={toggleShow}>
          confirm
        </Button>
      }
      content={
        <TextArea
          class="scroller"
          value={dotenv()}
          onEdit={setDotEnv}
          style={{ width: "40rem", "max-width": "90vw", height: "80vh" }}
          spellcheck={false}
        />
      }
    />
  );
};

export default Env;
