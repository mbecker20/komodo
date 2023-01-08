import { Component, createEffect, createSignal, Show } from "solid-js";
import { client } from "../../../../..";
import { useAppState } from "../../../../../state/StateProvider";
import { BuildVersionsReponse } from "../../../../../types";
import { combineClasses, string_to_version, version_to_string } from "../../../../../util/helpers";
import Input from "../../../../shared/Input";
import Flex from "../../../../shared/layout/Flex";
import Selector from "../../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const Image: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const { builds } = useAppState();
  const [versions, setVersions] = createSignal<BuildVersionsReponse[]>([]);
  createEffect(() => {
    if (deployment.build_id) {
      client.get_build_versions(deployment.build_id).then(setVersions);
    }
  });
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
              setDeployment("docker_run_args", { image: "" });
            }}
            position="bottom right"
            disabled={!userCanUpdate()}
            useSearch
          />
          <Show when={deployment.build_id}>
            <Selector
              targetClass="blue"
              selected={
                deployment.build_version
                  ? `v${version_to_string(deployment.build_version)}`
                  : "latest"
              }
              items={[
                "latest",
                ...versions().map((v) => `v${version_to_string(v.version)}`),
              ]}
              onSelect={(version) => {
                if (version === "latest") {
                  setDeployment("build_version", undefined);
                } else {
                  setDeployment(
                    "build_version",
                    string_to_version(version.replace("v", ""))
                  );
                }
              }}
              position="bottom right"
              disabled={!userCanUpdate()}
              useSearch
            />
          </Show>
        </Show>
      </Flex>
    </Flex>
  );
};

export default Image;
