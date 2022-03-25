import { Deployment } from "@monitor/types";
import { Component, createEffect, For, Show } from "solid-js";
import { DeepReadonly, SetStoreFunction } from "solid-js/store";
import { useAppState } from "../../../../state/StateProvider";
import { useToggle } from "../../../../util/hooks";
import Icon from "../../../util/icons/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Menu from "../../../util/menu/Menu";
import s from "../../deployment.module.css";

const Image: Component<{
  deployment: DeepReadonly<Deployment>;
  setDeployment: SetStoreFunction<Deployment>;
}> = (p) => {
  const { builds } = useAppState();
  const [show, toggle] = useToggle();
  createEffect(() => console.log(p.deployment));
  return (
    <Flex class={s.ConfigItem} justifyContent="space-between">
      <div class={s.ItemHeader}>{p.deployment.buildID ? "build" : "image"}</div>
      <Flex>
        <Menu
          show={show()}
          target={
            <button class="green" onClick={toggle}>
              {p.deployment.buildID
                ? builds.get(p.deployment.buildID)?.name
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
        <Show when={p.deployment.image}>
          <Flex>
            <Input
              placeholder="image"
              spellcheck={false}
              value={p.deployment.image}
              style={{ width: "12rem" }}
            />
          </Flex>
        </Show>
      </Flex>
    </Flex>
  );
};

export default Image;
