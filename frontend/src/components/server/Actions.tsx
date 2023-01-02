import { Component, Show } from "solid-js";
import { client, pushNotification } from "../..";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import ConfirmButton from "../shared/ConfirmButton";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import { combineClasses, getId } from "../../util/helpers";
import { useActionStates } from "./ActionStateProvider";
import Loading from "../shared/loading/Loading";
import { useParams } from "@solidjs/router";
import { PermissionLevel, ServerStatus } from "../../types";

const Actions: Component<{}> = (p) => {
  const { ws, servers } = useAppState();
  const params = useParams();
  const { user } = useUser();
  const server = () => servers.get(params.id)!;
  const userCanExecute = () =>
    user().admin ||
    server().server.permissions![getId(user())] === PermissionLevel.Execute ||
    server().server.permissions![getId(user())] === PermissionLevel.Update;
  return (
    <Show
      when={server() && server().status === ServerStatus.Ok && userCanExecute()}
    >
      <Grid class={combineClasses("card shadow")}>
        <h1>actions</h1>
        <Flex class={combineClasses("action shadow")}>
          prune images <PruneImages />
        </Flex>
        <Flex class={combineClasses("action shadow")}>
          prune containers <PruneContainers />
        </Flex>
        <Flex class={combineClasses("action shadow")}>
          prune networks{" "}
          <ConfirmButton
            color="green"
            onConfirm={() => {
              client.prune_docker_networks(params.id);
            }}
          >
            <Icon type="cut" />
          </ConfirmButton>
        </Flex>
      </Grid>
    </Show>
  );
};

export default Actions;

function PruneImages() {
  const params = useParams();
  const actions = useActionStates();
  return (
    <Show
      when={!actions.pruning_images}
      fallback={
        <button class="green">
          <Loading type="spinner" />
        </button>
      }
    >
      <ConfirmButton
        color="green"
        onConfirm={() => {
          client.prune_docker_images(params.id);
        }}
      >
        <Icon type="cut" />
      </ConfirmButton>
    </Show>
  );
}

function PruneContainers() {
  const params = useParams();
  const actions = useActionStates();
  return (
    <Show
      when={!actions.pruning_containers}
      fallback={
        <button class="blue">
          <Loading type="spinner" />
        </button>
      }
    >
      <ConfirmButton
        color="blue"
        onConfirm={() => {
          client.prune_docker_containers(params.id);
        }}
      >
        <Icon type="cut" />
      </ConfirmButton>
    </Show>
  );
}
