import { Component, Show } from "solid-js";
import { useAppState } from "../../../../../state/StateProvider";
import { combineClasses } from "../../../../../util/helpers";
import Input from "../../../../shared/Input";
import Flex from "../../../../shared/layout/Flex";
import Selector from "../../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const Image: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const { builds } = useAppState();
  return (
    <Flex
      class={combineClasses("config-item shadow")}
      justifyContent="space-between"
    >
      <h1>{deployment.build_id ? "build" : "image"}</h1>
      <Flex alignItems="center" style={{ "flex-wrap": "wrap" }}>
        <Show when={!deployment.build_id}>
          <Input
            placeholder="image"
            spellcheck={false}
            value={deployment.docker_run_args.image || ""}
            style={{ width: userCanUpdate() ? "12rem" : undefined }}
            onEdit={(image) => setDeployment("docker_run_args", { image })}
            disabled={!userCanUpdate()}
          />
        </Show>
        <Show
          when={builds.loaded() && (userCanUpdate() || deployment.build_id)}
        >
          <Selector
            targetClass="blue"
            selected={
              (deployment.build_id && builds.get(deployment.build_id)?.name) ||
              "custom image"
            }
            items={[
              "custom image",
              ...builds
                .ids()!
                .map((id) => builds.get(id)?.name!)
                .filter((val) => val),
            ]}
            onSelect={(build) => {
              setDeployment(
                "build_id",
                build === "custom image"
                  ? undefined
                  : builds.ids()!.find((id) => builds.get(id)?.name === build)
              );
            }}
            position="bottom right"
            disabled={!userCanUpdate()}
            useSearch
          />
        </Show>
      </Flex>
    </Flex>
  );
};

export default Image;
