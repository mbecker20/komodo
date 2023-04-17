import { Component, createResource, Show } from "solid-js";
import { client } from "../../../../..";
import { useAppState } from "../../../../../state/StateProvider";
import {
  combineClasses,
  readableVersion,
  readableMonitorTimestamp,
} from "../../../../../util/helpers";
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
              (deployment.build_id &&
                (builds.get(deployment.build_id)?.name || "unknown")) ||
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
            <VersionSelector />
          </Show>
        </Show>
      </Flex>
    </Flex>
  );
};

export default Image;


const VersionSelector: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const [versions] = createResource(() => {
    if (deployment.build_id) {
      return client.get_build_versions(deployment.build_id);
    }
  });
  const selected = () => ({
    version: deployment.build_version || {
      major: 0,
      minor: 0,
      patch: 0,
    },
    ts: "",
  });
  return (
    <Selector
      targetClass="blue"
      selected={selected()}
      items={[
        { version: { major: 0, minor: 0, patch: 0 }, ts: "" },
        ...(versions() || []),
      ]}
      itemMap={({ version, ts }) => (
        <>
          <div>
            {version.major === 0 && version.minor === 0 && version.patch === 0
              ? "latest"
              : readableVersion(version)}
          </div>
          <Show when={ts.length > 0}>
            <div class="dimmed">{readableMonitorTimestamp(ts)}</div>
          </Show>
        </>
      )}
      searchItemMap={({ version }) => readableVersion(version)}
      onSelect={({ version, ts }) => {
        if (ts.length === 0) {
          setDeployment("build_version", undefined);
        } else {
          setDeployment("build_version", version);
        }
      }}
      position="bottom right"
      disabled={!userCanUpdate()}
      useSearch
    />
  );
}