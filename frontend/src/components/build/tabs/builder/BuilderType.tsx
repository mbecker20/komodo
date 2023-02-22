import { Component, Show } from "solid-js";
import { useAppState } from "../../../../state/StateProvider";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
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
        <Grid gap="0" gridTemplateColumns="1fr 1fr">
          <button
            class={builderType() === "server" ? "blue" : "grey"}
            style={{ width: "100%" }}
            onClick={() => {
              if (builderType() !== "server") {
                const server_id =
                  servers.ids()?.length || 0 > 0
                    ? servers.ids()![0]
                    : undefined;
                setBuild({ server_id, aws_config: undefined });
              }
            }}
          >
            server
          </button>
          <button
            class={builderType() === "aws" ? "blue" : "grey"}
            style={{ width: "100%" }}
            onClick={() => {
              if (builderType() !== "aws") {
                setBuild({ server_id: undefined, aws_config: {} });
              }
            }}
          >
            aws
          </button>
        </Grid>
      </Show>
    </Flex>
  );
};

export default BuilderType;
