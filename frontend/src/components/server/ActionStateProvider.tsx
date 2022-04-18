import { ServerActionState } from "@monitor/types";
import {
  Component,
  createContext,
  createEffect,
  onCleanup,
  useContext,
} from "solid-js";
import { createStore } from "solid-js/store";
import { PRUNE_IMAGES } from "@monitor/util";
import { useAppState } from "../../state/StateProvider";
import { getDeploymentActionState } from "../../util/query";

type State = {} & ServerActionState;

const context = createContext<State>();

export const ActionStateProvider: Component<{}> = (p) => {
  const { selected, ws } = useAppState();
  const [actions, setActions] = createStore<ServerActionState>({
    pruningImages: false,
		pruningNetworks: false,
		deleting: false,
  });
  createEffect(() => {
    getDeploymentActionState(selected.id()).then(setActions);
  });
  onCleanup(
    ws.subscribe([PRUNE_IMAGES], ({ complete, serverID }) => {
      if (serverID === selected.id()) {
        setActions("pruningImages", !complete);
      }
    })
  );
  return <context.Provider value={actions}>{p.children}</context.Provider>;
};

export function useActionStates() {
  return useContext(context) as State;
}
