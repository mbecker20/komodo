import { ServerActionState } from "@monitor/types";
import {
  Component,
  createContext,
  createEffect,
  onCleanup,
  useContext,
} from "solid-js";
import { createStore } from "solid-js/store";
import { PRUNE_CONTAINERS, PRUNE_IMAGES, PRUNE_NETWORKS } from "@monitor/util";
import { useAppState } from "../../state/StateProvider";
import { getServerActionState } from "../../util/query";

type State = {} & ServerActionState;

const context = createContext<State>();

export const ActionStateProvider: Component<{}> = (p) => {
  const { selected, ws } = useAppState();
  const [actions, setActions] = createStore<ServerActionState>({
    pruningImages: false,
		pruningNetworks: false,
    pruningContainers: false,
		deleting: false,
  });
  createEffect(() => {
    getServerActionState(selected.id()).then(setActions);
  });
  onCleanup(
    ws.subscribe([PRUNE_IMAGES], ({ complete, serverID }) => {
      if (serverID === selected.id()) {
        setActions("pruningImages", !complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([PRUNE_NETWORKS], ({ complete, serverID }) => {
      if (serverID === selected.id()) {
        setActions("pruningNetworks", !complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([PRUNE_CONTAINERS], ({ complete, serverID }) => {
      if (serverID === selected.id()) {
        setActions("pruningContainers", !complete);
      }
    })
  );
  return <context.Provider value={actions}>{p.children}</context.Provider>;
};

export function useActionStates() {
  return useContext(context) as State;
}
