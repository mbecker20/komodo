import { ContainerStatus } from "@monitor/types";
import { Component, Show } from "solid-js";
import { DELETE_DEPLOYMENT } from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { deploymentHeaderStatusClass } from "../../util/helpers";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import Loading from "../util/loading/Loading";
import HoverMenu from "../util/menu/HoverMenu";
import { useActionStates } from "./ActionStateProvider";

const Header: Component<{}> = (p) => {
  const { deployments, ws, selected } = useAppState();
  const deployment = () => deployments.get(selected.id())!;
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
  return (
    <Grid gap="0.5rem" class="card shadow">
      <Flex alignItems="center" justifyContent="space-between">
        <h1>{deployment()!.name}</h1>
        <Show
          when={permissions() >= 2 || deployment().owners.includes(username()!)}
        >
          <Show
            when={!actions.fullDeleting}
            fallback={
              <button class="red">
                <Icon type="trash" />
              </button>
            }
          >
            <HoverMenu
              target={
                <Show
                  when={!actions.fullDeleting}
                  fallback={
                    <button class="red">
                      <Loading />
                    </button>
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
        <div class={deploymentHeaderStatusClass(state())}>{state()}</div>
        <Show when={status()}>
          <div style={{ opacity: 0.7 }}>{status()}</div>
        </Show>
      </Flex>
    </Grid>
  );
};

export default Header;
