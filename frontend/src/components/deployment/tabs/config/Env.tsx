import { Component, For, Show } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Icon from "../../../util/icons/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import s from "../../deployment.module.css";
import { useConfig } from "./Provider";

const Env: Component<{}> = (p) => {
  const { deployment, setDeployment } = useConfig();
  const onAdd = () => {
    setDeployment("environment", (environment: any) => [
      ...environment,
      { variable: "", value: "" },
    ]);
  };
  const onRemove = (index: number) => {
    setDeployment("environment", (environment) =>
      environment!.filter((_, i) => i !== index)
    );
  };
  return (
    <Grid class={combineClasses(s.ConfigItem, "shadow")}>
      <Flex alignItems="center">
        <h1>environment</h1>
        <Show
          when={!deployment.environment || deployment.environment.length === 0}
        >
          <div>none</div>
        </Show>
        <button onClick={onAdd}>
          <Icon type="plus" />
        </button>
      </Flex>
      <For each={deployment.environment}>
        {({ variable, value }, index) => (
          <Flex justifyContent="center">
            <Input
              placeholder="variable"
              value={variable}
              style={{ width: "40%" }}
              onConfirm={(value) =>
                setDeployment("environment", index(), "variable", value)
              }
            />
            {" : "}
            <Input
              placeholder="value"
              value={value}
              style={{ width: "40%" }}
              onConfirm={(value) =>
                setDeployment("environment", index(), "value", value)
              }
            />
            <button onClick={() => onRemove(index())}>
              <Icon type="minus" />
            </button>
          </Flex>
        )}
      </For>
    </Grid>
  );
};

export default Env;
