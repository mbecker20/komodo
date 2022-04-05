import { DeployActionState } from "@monitor/types";
import {
  Component,
  createContext,
  createEffect,
  onCleanup,
  useContext,
} from "solid-js";
import { createStore } from "solid-js/store";
import { DEPLOY } from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { getDeploymentActionState } from "../../util/query";

type State = {} & DeployActionState;

const context = createContext<State>();

export const ActionStateProvider: Component<{}> = (p) => {
  const { selected, ws } = useAppState();
  const [actions, setActions] = createStore<DeployActionState>({
    deploying: false,
    deleting: false,
    starting: false,
    stopping: false,
    fullDeleting: false,
    updating: false,
  });
  createEffect(() => {
    getDeploymentActionState(selected.id()).then(setActions);
  });
  onCleanup(
    ws.subscribe([DEPLOY], ({ complete, deploymentID }) => {
      if (deploymentID === selected.id()) {
        setActions("deploying", !complete);
      }
    })
  );
  return <context.Provider value={actions}>{p.children}</context.Provider>;
};

export function useActionStates() {
  return useContext(context) as State;
}
