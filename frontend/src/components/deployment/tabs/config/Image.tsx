import { Component, For, Show } from "solid-js";
import { useAppState } from "../../../../state/StateProvider";
import { combineClasses } from "../../../../util/helpers";
import { useToggle } from "../../../../util/hooks";
import Icon from "../../../util/icons/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Menu from "../../../util/menu/Menu";
import Selector from "../../../util/menu/Selector";
import s from "../../deployment.module.css";
import { useConfig } from "./Provider";

const Image: Component<{}> = (p) => {
  const { deployment, setDeployment } = useConfig();
  const { builds } = useAppState();
  const [show, toggle] = useToggle();
  return (
    <Flex
      class={combineClasses(s.ConfigItem, "shadow")}
      justifyContent="space-between"
    >
      <h1>{deployment.buildID ? "build" : "image"}</h1>
      <Flex>
        <Show when={!deployment.buildID}>
          <Flex>
            <Input
              placeholder="image"
              spellcheck={false}
              value={deployment.image || ""}
              style={{ width: "12rem" }}
              onEdit={(value) => setDeployment("image", value)}
            />
          </Flex>
        </Show>
        <Show when={builds.loaded()}>
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
            position="bottom center"
          />
        </Show>
      </Flex>
    </Flex>
  );
};

export default Image;
