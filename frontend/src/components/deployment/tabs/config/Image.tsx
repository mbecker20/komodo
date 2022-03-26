import { Component, For, Show } from "solid-js";
import { useAppState } from "../../../../state/StateProvider";
import { combineClasses } from "../../../../util/helpers";
import { useToggle } from "../../../../util/hooks";
import Icon from "../../../util/icons/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Menu from "../../../util/menu/Menu";
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
      <div class={s.ItemHeader}>{deployment.buildID ? "build" : "image"}</div>
      <Flex>
        <Menu
          show={show()}
          target={
            <button class="green" onClick={toggle}>
              {deployment.buildID
                ? builds.get(deployment.buildID)?.name
                : "custom image"}
              <Icon type="chevron-down" />
            </button>
          }
          content={
            <Grid>
              <button class="green">custom image</button>
              <For each={builds.ids()}>
                {(buildID) => (
                  <button class="blue">{builds.get(buildID)?.name}</button>
                )}
              </For>
            </Grid>
          }
          position="bottom center"
        />
        <Show when={!deployment.buildID}>
          <Flex>
            <Input
              placeholder="image"
              spellcheck={false}
              value={deployment.image || ""}
              style={{ width: "12rem" }}
              onConfirm={(value) => setDeployment("image", value)}
            />
          </Flex>
        </Show>
      </Flex>
    </Flex>
  );
};

export default Image;
