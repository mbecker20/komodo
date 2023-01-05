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
import { BuildActionState, DeploymentActionState, Operation, UpdateStatus } from "../../types";

type State = DeploymentActionState & Partial<BuildActionState>;

const context = createContext<State>();

export const ActionStateProvider: ParentComponent<{ exiting?: boolean }> = (p) => {
  const { deployments, builds, ws } = useAppState();
  const params = useParams();
  const [actions, setActions] = createStore<
    DeploymentActionState & Partial<BuildActionState>
  >({
    deploying: false,
    starting: false,
    stopping: false,
    removing: false,
    updating: false,
    pulling: false,
    recloning: false,
    building: false,
  });
  const deployment = () => deployments.get(params.id)!
  createEffect(() => {
    client.get_deployment_action_state(params.id).then(setActions);
    const buildID = deployment().deployment.build_id;
    if (buildID && builds.get(buildID)) {
      client.get_build_action_state(buildID).then((state) => {
        setActions("building", state.building);
      });
    }
  });
  onCleanup(
    ws.subscribe([Operation.DeployContainer], (update) => {
      if (update.target.id === params.id) {
        setActions("deploying", update.status !== UpdateStatus.Complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([Operation.RemoveContainer], (update) => {
      if (update.target.id === params.id) {
        setActions("removing", update.status !== UpdateStatus.Complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([Operation.StartContainer], (update) => {
      if (update.target.id === params.id) {
        setActions("starting", update.status !== UpdateStatus.Complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([Operation.StopContainer], (update) => {
      if (update.target.id === params.id) {
        setActions("stopping", update.status !== UpdateStatus.Complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([Operation.DeleteDeployment], (update) => {
      // if (update.target.id === params.id) {
      //   setActions("deploying", update.status !== UpdateStatus.Complete);
      // }
    })
  );
  onCleanup(
    ws.subscribe([Operation.PullDeployment], (update) => {
      if (update.target.id === params.id) {
        setActions("pulling", update.status !== UpdateStatus.Complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([Operation.RecloneDeployment], (update) => {
      if (update.target.id === params.id) {
        setActions("recloning", update.status !== UpdateStatus.Complete);
      }
    })
  );
  onCleanup(ws.subscribe([Operation.BuildBuild], (update) => {
    if (deployment().deployment.build_id === update.target.id) {
      setActions("building", update.status !== UpdateStatus.Complete);
    }
  }));
  return <context.Provider value={actions}>{p.children}</context.Provider>;
};

export function useActionStates() {
  return useContext(context) as State;
}
