import { Component, createEffect, createMemo, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useTheme } from "../../../state/ThemeProvider";
import { useUser } from "../../../state/UserProvider";
import { combineClasses } from "../../../util/helpers";
import { useLocalStorageToggle } from "../../../util/hooks";
import Button from "../../util/Button";
import Icon from "../../util/Icon";
import Flex from "../../util/layout/Flex";
import Grid from "../../util/layout/Grid";
import s from "../sidebar.module.scss";
import Deployment from "./Deployment";
import NewDeployment from "./NewDeployment";

const Server: Component<{ id: string }> = (p) => {
  const { servers, deployments, selected } = useAppState();
  const { permissions, username } = useUser();
  const server = () => servers.get(p.id);
  const deploymentIDs = createMemo(() => {
    return (
      deployments.loaded() &&
      deployments.ids()!.filter((id) => deployments.get(id)?.serverID === p.id)
    );
  });
  const [open, toggleOpen] = useLocalStorageToggle(p.id);
  const { themeClass } = useTheme();
  return (
    <Show when={server()}>
      <div class={combineClasses(s.Server, "shadow", themeClass())}>
        <Button
          class={combineClasses(
            s.ServerButton,
            selected.id() === p.id && "selected",
            "shadow"
          )}
          onClick={toggleOpen}
        >
          <Flex>
            <Icon type="chevron-down" width="1rem" />
            <h2 style={{ opacity: 0.7 }}>{server()?.name}</h2>
          </Flex>
          <div
            class={server()?.status === "OK" ? "green" : "red"}
            style={{
              padding: "0.25rem",
              "border-radius": ".35rem",
              transition: "background-color 125ms ease-in-out",
            }}
            onClick={(e) => {
              e.stopPropagation();
              selected.set(p.id, "server");
            }}
          >
            {server()?.status === "OK" ? "OK" : "NOT OK"}
          </div>
        </Button>
        <Show when={open()}>
          <Grid
            gap=".5rem"
            class={combineClasses(
              s.Deployments,
              open() ? s.Enter : s.Exit,
              themeClass()
            )}
          >
            <For each={deploymentIDs()}>{(id) => <Deployment id={id} />}</For>
            <Show
              when={
                permissions() > 1 ||
                (permissions() > 0 && server()!.owners.includes(username()!))
              }
            >
              <NewDeployment serverID={p.id} />
            </Show>
          </Grid>
        </Show>
      </div>
    </Show>
  );
};

export default Server;
