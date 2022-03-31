import { Component, For, Show } from "solid-js";
import Icon from "../../../util/icons/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
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
    <Grid class="config-item shadow">
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
        {(_, index) => (
          <Flex justifyContent="center">
            <Input
              placeholder="variable"
              value={deployment.environment![index()].variable}
              style={{ width: "40%" }}
              onEdit={(value) =>
                setDeployment(
                  "environment",
                  index(),
                  "variable",
                  value.toUpperCase().replaceAll(" ", "_")
                )
              }
            />
            {" : "}
            <Input
              placeholder="value"
              value={deployment.environment![index()].value}
              style={{ width: "40%" }}
              onEdit={(value) =>
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
