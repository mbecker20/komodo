import { Component, Show } from "solid-js";
import { useAppState } from "../../../../state/StateProvider";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import Selector from "../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const BuilderType: Component<{}> = (p) => {
	const { servers } = useAppState();
  const { build, setBuild, userCanUpdate } = useConfig();
  const builderType = () => {
    if (build.server_id) {
      return "server";
    } else if (build.aws_config) {
      return "aws";
    } else {
      return undefined;
    }
  };
  return (
    <Flex
      class="config-item shadow"
      alignItems="center"
      justifyContent="space-between"
    >
      <h1>builder type</h1>
      <Show when={userCanUpdate()} fallback={<h2>{builderType()}</h2>}>
        <Selector 
          targetClass="blue"
          selected={builderType() || "select type"}
          items={["aws", "server"]}
          position="bottom right"
          onSelect={(type) => {
            if (type !== builderType()) {
              if (type === "server") {
                const server_id =
                  servers.ids()?.length || 0 > 0
                    ? servers.ids()![0]
                    : undefined;
                setBuild({ server_id, aws_config: undefined });
              } else if (type === "aws") {
                setBuild({ server_id: undefined, aws_config: {} });
              }
            }
          }}
        />
      </Show>
    </Flex>
  );
};

export default BuilderType;
