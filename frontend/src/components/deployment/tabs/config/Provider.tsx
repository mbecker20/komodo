import { Deployment, Network, Update } from "@monitor/types";
import {
  Accessor,
  Component,
  createContext,
  createEffect,
  createSignal,
  onCleanup,
  useContext,
} from "solid-js";
import { createStore, DeepReadonly, SetStoreFunction } from "solid-js/store";
import { ADD_UPDATE, UPDATE_DEPLOYMENT } from "../../../../state/actions";
import { useAppState } from "../../../../state/StateProvider";
import { useUser } from "../../../../state/UserProvider";
import { getDeployment, getNetworks } from "../../../../util/query";

type ConfigDeployment = Deployment & {
  loaded: boolean;
  updated: boolean;
  updating: boolean;
};

type State = {
  editing: Accessor<boolean>;
  deployment: DeepReadonly<ConfigDeployment>;
  setDeployment: SetStoreFunction<ConfigDeployment>;
  reset: () => void;
  save: () => void;
  networks: Accessor<Network[]>;
  userCanUpdate: () => boolean;
};

const context = createContext<State>();

export const ConfigProvider: Component<{}> = (p) => {
  const { ws, deployments, selected } = useAppState();
  const { username, permissions } = useUser();
  const [editing] = createSignal(false);
  const [deployment, set] = createStore({
    ...deployments.get(selected.id())!,
    loaded: false,
    updated: false,
    updating: false,
  });
  const setDeployment = (...args: any) => {
    // @ts-ignore
    set(...args);
    set("updated", true);
  };
  const load = () => {
    console.log("loading deployment");
    getDeployment(selected.id()).then((deployment) =>
      set({
        ...deployment,
        image: deployment.image,
        network: deployment.network,
        buildID: deployment.buildID,
        dockerAccount: deployment.dockerAccount,
        githubAccount: deployment.githubAccount,
        repo: deployment.repo,
        branch: deployment.branch,
        onPull: deployment.onPull,
        onClone: deployment.onClone,
        containerMount: deployment.containerMount,
        repoMount: deployment.repoMount,
        loaded: true,
        updated: false,
        updating: false,
      })
    );
  };
  createEffect(load);

  const [networks, setNetworks] = createSignal<Network[]>([]);
  createEffect(() => {
    console.log("load networks");
    getNetworks(deployments.get(selected.id())!.serverID!).then(setNetworks);
  });

  const save = () => {
    setDeployment("updating", true);
    ws.send(UPDATE_DEPLOYMENT, { deployment });
  };

  const unsub = ws.subscribe([ADD_UPDATE], ({ update }: { update: Update }) => {
    if (update.deploymentID === selected.id()) {
      if ([UPDATE_DEPLOYMENT].includes(update.operation)) {
        load();
      }
    }
  });

  onCleanup(unsub);

  const userCanUpdate = () => {
    if (permissions() > 1) {
      return true;
    } else if (permissions() > 0 && deployment.owners.includes(username()!)) {
      return true;
    } else {
      return false;
    }
  };

  const state = {
    editing,
    deployment,
    setDeployment,
    reset: load,
    save,
    networks,
    userCanUpdate,
  };

  return <context.Provider value={state}>{p.children}</context.Provider>;
};

export function useConfig() {
  return useContext(context) as State;
}
