import { useParams } from "@solidjs/router";
import {
  createContext,
  createEffect,
  onCleanup,
  ParentComponent,
  useContext,
} from "solid-js";
import { createStore } from "solid-js/store";
import { client } from "../..";
import { useAppState } from "../../state/StateProvider";
import { Operation, ServerActionState, UpdateStatus } from "../../types";

type State = {} & ServerActionState;

const context = createContext<State>();

export const ActionStateProvider: ParentComponent<{}> = (p) => {
  const params = useParams();
  const { ws } = useAppState();
  const [actions, setActions] = createStore<ServerActionState>({
    pruning_networks: false,
    pruning_containers: false,
    pruning_images: false,
  });
  createEffect(() => {
    client.get_server_action_state(params.id).then(setActions);
  });
  onCleanup(
    ws.subscribe([Operation.PruneImagesServer], (update) => {
      if (update.target.id === params.id) {
        setActions("pruning_images", update.status !== UpdateStatus.Complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([Operation.PruneNetworksServer], (update) => {
      if (update.target.id === params.id) {
        setActions("pruning_networks", update.status !== UpdateStatus.Complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([Operation.PruneContainersServer], (update) => {
      if (update.target.id === params.id) {
        setActions(
          "pruning_containers",
          update.status !== UpdateStatus.Complete
        );
      }
    })
  );
  return <context.Provider value={actions}>{p.children}</context.Provider>;
};

export function useActionStates() {
  return useContext(context) as State;
}
