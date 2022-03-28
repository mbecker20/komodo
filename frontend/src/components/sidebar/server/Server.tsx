import { Component, createEffect, createMemo, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { combineClasses } from "../../../util/helpers";
import { useLocalStorageToggle } from "../../../util/hooks";
import { getDeployments } from "../../../util/query";
import CreateDeployment from "../../create/Deployment";
import Icon from "../../util/icons/Icon";
import Flex from "../../util/layout/Flex";
import Grid from "../../util/layout/Grid";
import s from "../sidebar.module.css";
import Deployment from "./Deployment";

const Server: Component<{ id: string }> = (p) => {
  const { servers, deployments, selected } = useAppState();
  const server = () => servers.get(p.id);
  const deploymentIDs = createMemo(() => {
    return (
      deployments.loaded() &&
      deployments.ids()!.filter((id) => deployments.get(id)?.serverID === p.id)
    );
  });
  const [open, toggleOpen] = useLocalStorageToggle(p.id);
  createEffect(() => {
    if (server() && !server()!.isCore) {
      getDeployments({ serverID: p.id }).then((more) =>
        deployments.addMany(more)
      );
    }
  });
  return (
    <Show when={server()}>
      <div class={combineClasses(s.Server, "shadow")}>
        <button
          class={combineClasses(
            s.ServerButton,
            "shadow",
            selected.id() === p.id && "selected"
          )}
          onClick={toggleOpen}
        >
          <Flex>
            <Icon type="chevron-down" width="1rem" />
            <div>{server()?.name}</div>
          </Flex>
          <div
            class={server()?.status === "OK" ? "green" : "red"}
            style={{ padding: "0.25rem", "border-radius": ".35rem" }}
            onClick={(e) => {
              e.stopPropagation();
              selected.set(p.id, "server");
            }}
          >
            {server()?.status === "OK" ? "OK" : "DISCONNECTED"}
          </div>
        </button>
        <Show when={open()}>
          <Grid
            gap=".5rem"
            class={combineClasses(s.Deployments, open() ? s.Enter : s.Exit)}
          >
            <For each={deploymentIDs()}>{(id) => <Deployment id={id} />}</For>
            <CreateDeployment serverID={p.id} />
          </Grid>
        </Show>
      </div>
    </Show>
  );
};

export default Server;
