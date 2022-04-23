import { ContainerStatus } from "@monitor/types";
import { Component, Show } from "solid-js";
import { DELETE_DEPLOYMENT } from "@monitor/util";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import {
  combineClasses,
  deploymentHeaderStatusClass,
} from "../../util/helpers";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import Loading from "../util/loading/Loading";
import HoverMenu from "../util/menu/HoverMenu";
import { useActionStates } from "./ActionStateProvider";
import { useTheme } from "../../state/ThemeProvider";
import Button from "../util/Button";
import { useLocalStorageToggle } from "../../util/hooks";
import { useAppDimensions } from "../../state/DimensionProvider";
import Updates from "./Updates";

const Header: Component<{ exiting?: boolean }> = (p) => {
  const { deployments, ws, selected } = useAppState();
  const deployment = p.exiting
    ? () => deployments.get(selected.prevId()!)!
    : () => deployments.get(selected.id())!;
  const { permissions, username } = useUser();
  const state = () =>
    deployment()!.status === "not deployed"
      ? "not deployed"
      : (deployment()!.status as ContainerStatus).State;
  const status = () =>
    deployment()!.status === "not deployed"
      ? undefined
      : (deployment()!.status as ContainerStatus).Status.toLowerCase();
  const actions = useActionStates();
  const { themeClass } = useTheme();
  const { isMobile } = useAppDimensions();
  const [showUpdates, toggleShowUpdates] =
    useLocalStorageToggle("show-updates");
  return (
    <>
      <Grid
        gap="0.5rem"
        class={combineClasses("card shadow", themeClass())}
        style={{
          position: "relative",
          cursor: isMobile() ? "pointer" : undefined,
        }}
        onClick={() => {
          if (isMobile()) toggleShowUpdates();
        }}
      >
        <Flex alignItems="center" justifyContent="space-between">
          <h1>{deployment()!.name}</h1>
          <Show
            when={
              permissions() >= 2 || deployment().owners.includes(username()!)
            }
          >
            <Show
              when={!actions.fullDeleting}
              fallback={
                <Button class="red">
                  <Icon type="trash" />
                </Button>
              }
            >
              <HoverMenu
                target={
                  <Show
                    when={!actions.fullDeleting}
                    fallback={
                      <Button class="red">
                        <Loading />
                      </Button>
                    }
                  >
                    <ConfirmButton
                      onConfirm={() => {
                        ws.send(DELETE_DEPLOYMENT, {
                          deploymentID: selected.id(),
                        });
                      }}
                      color="red"
                    >
                      <Icon type="trash" />
                    </ConfirmButton>
                  </Show>
                }
                content="delete deployment"
                position="bottom center"
                padding="0.5rem"
              />
            </Show>
          </Show>
        </Flex>
        <Flex alignItems="center" justifyContent="space-between">
          <div class={deploymentHeaderStatusClass(state(), themeClass)}>
            {state()}
          </div>
          <Show when={status()}>
            <div style={{ opacity: 0.7 }}>{status()}</div>
          </Show>
        </Flex>
        <Show when={isMobile()}>
          <Flex gap="0.5rem" alignItems="center" class="show-updates-indicator">
            updates{" "}
            <Icon
              type={showUpdates() ? "chevron-up" : "chevron-down"}
              width="0.9rem"
            />
          </Flex>
        </Show>
      </Grid>
      <Show when={isMobile() && showUpdates()}>
        <Updates />
      </Show>
    </>
  );
};

export default Header;
