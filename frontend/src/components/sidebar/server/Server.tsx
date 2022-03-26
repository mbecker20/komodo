import { Component, createMemo, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { combineClasses } from "../../../util/helpers";
import { useLocalStorageToggle } from "../../../util/hooks";
import Icon from "../../util/icons/Icon";
import Flex from "../../util/layout/Flex";
import Grid from "../../util/layout/Grid";
import s from "../sidebar.module.css";
import Deployment from "./Deployment";

const Server: Component<{ id: string }> = (p) => {
  const { servers, deployments } = useAppState();
  const server = () => servers.get(p.id);
  const deploymentIDs = createMemo(() => {
    return (
      deployments.loaded() &&
      deployments.ids()!.filter((id) => deployments.get(id)?.serverID === p.id)
    );
  });
  const [open, toggleOpen] = useLocalStorageToggle(p.id);
  return (
    <div class={combineClasses(s.Server, "shadow")}>
      <button
        class={combineClasses(s.ServerButton, "shadow")}
        onClick={toggleOpen}
      >
        <Flex>
          <Icon type="chevron-down" width="1rem" />
          <div>{server()?.name}</div>
        </Flex>
        <div>{server()?.status}</div>
      </button>
      <Show when={open()}>
        <Grid gap=".15rem" class={s.Deployments}>
          <For each={deploymentIDs()}>{(id) => <Deployment id={id} />}</For>
          <button class={combineClasses("green", s.CreateButton)}>
            <Icon type="plus" />
          </button>
        </Grid>
      </Show>
    </div>
  );
};

export default Server;
