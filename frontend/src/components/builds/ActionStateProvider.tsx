import { BuildActionState } from "@monitor/types";
import { Component, createContext, createEffect, onCleanup, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { BUILD, CLONE_BUILD_REPO, DELETE_BUILD } from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { getBuildActionState } from "../../util/query";

type State = {
  
} & BuildActionState;

const context = createContext<State>();

export const ActionStateProvider: Component<{}> = (p) => {
	const { selected, ws } = useAppState();
  const [actions, setActions] = createStore<BuildActionState>({
    pulling: false,
    building: false,
    cloning: false,
    updating: false,
		deleting: false,
  });
	createEffect(() => {
    getBuildActionState(selected.id()).then(setActions);
  });
  onCleanup(
    ws.subscribe([BUILD], ({ complete, buildID }) => {
      if (buildID === selected.id()) {
        setActions("building", !complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([CLONE_BUILD_REPO], ({ complete, buildID }) => {
      if (buildID === selected.id()) {
        setActions("cloning", !complete);
      }
    })
  );
	onCleanup(
    ws.subscribe([DELETE_BUILD], ({ complete, buildID }) => {
      if (buildID === selected.id()) {
        setActions("deleting", !complete);
      }
    })
  );
	return (
		<context.Provider value={actions}>{p.children}</context.Provider>
	);
}

export function useActionStates() {
  return useContext(context) as State;
}