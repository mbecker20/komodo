import { Component, Show } from "solid-js";
import { useAppState } from "../../../../../state/StateProvider";
import { useUser } from "../../../../../state/UserProvider";
// import { useToggle } from "../../../../util/hooks";
import Input from "../../../../util/Input";
import Flex from "../../../../util/layout/Flex";
import Selector from "../../../../util/menu/Selector";
import { useConfig } from "../Provider";

const Image: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const { builds } = useAppState();
  // const [show, toggle] = useToggle();
  return (
    <Flex class="config-item shadow" justifyContent="space-between">
      <h1>{deployment.buildID ? "build" : "image"}</h1>
      <Flex alignItems="center" style={{ "flex-wrap": "wrap" }}>
        <Show when={!deployment.buildID}>
          <Input
            placeholder="image"
            spellcheck={false}
            value={deployment.image || ""}
            style={{ width: userCanUpdate() && "12rem" }}
            onEdit={(value) => setDeployment("image", value)}
            disabled={!userCanUpdate()}
          />
        </Show>
        <Show when={builds.loaded() && (userCanUpdate() || deployment.buildID)}>
          <Selector
            targetClass="blue"
            selected={
              (deployment.buildID && builds.get(deployment.buildID)?.name) ||
              "custom image"
            }
            items={[
              "custom image",
              ...builds
                .ids()!
                .map((id) => builds.get(id)?.name!)
                .filter((val) => val),
            ]}
            onSelect={(build, index) => {
              setDeployment(
                "buildID",
                build === "custom image" ? undefined : builds.ids()![index - 1]
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
