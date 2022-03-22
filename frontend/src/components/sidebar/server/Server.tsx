import { Component, createMemo, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useLocalStorageToggle } from "../../../util/hooks";
import Icon from "../../util/icons/Icon";
import Flex from "../../util/layout/Flex";
import s from "../sidebar.module.css";
import Deployment from "./Deployment";

const Server: Component<{ id: string }> = (p) => {
  const { servers, deployments } = useAppState();
  const server = () => servers.get(p.id);
  const deploymentIDs = createMemo(() => {
    return Object.keys(deployments.collection()!).filter(
      (id) => deployments.get(id)?.serverID === p.id
    );
  });
  const [open, toggleOpen] = useLocalStorageToggle(false, p.id);
  return (
    <div class={s.Server}>
      <Flex justifyContent="space-between" onClick={toggleOpen}>
        <Flex>
          <Icon type="chevron-down" alt="" width="1rem" />
          <div>{server()?.name}</div>
        </Flex>
        <div>{server()?.status}</div>
      </Flex>
      <Show when={open()}>
        <For each={deploymentIDs()}>{(id) => <Deployment id={id} />}</For>
      </Show>
    </div>
  );
};

export default Server;
