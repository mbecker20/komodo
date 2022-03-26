import { Deployment, Network } from "@monitor/types";
import {
  Accessor,
  Component,
  createContext,
  createEffect,
  createSignal,
  useContext,
} from "solid-js";
import { createStore, DeepReadonly, SetStoreFunction } from "solid-js/store";
import { getDeployment, getNetworks } from "../../../../util/query";

type ConfigDeployment = Deployment & { loaded: boolean; updated: boolean };

type State = {
  editing: Accessor<boolean>;
  deployment: DeepReadonly<ConfigDeployment>;
  setDeployment: SetStoreFunction<ConfigDeployment>;
  reset: () => void;
  networks: Accessor<Network[]>;
};

const context = createContext<State>();

export const ConfigProvider: Component<{ deployment: Deployment }> = (p) => {
  const [editing] = createSignal(false);
  const [deployment, set] = createStore({
    ...p.deployment,
    loaded: false,
    updated: false,
  });
  const setDeployment = (...args: any) => {
    // @ts-ignore
    set(...args);
    set("updated", true);
  };
  const load = () => {
    getDeployment(p.deployment._id!).then((deployment) =>
      set({
        ...deployment,
        image: deployment.image,
        network: deployment.network,
        buildID: deployment.buildID,
        loaded: true,
        updated: false,
      })
    );
  };
  createEffect(load);

  const [networks, setNetworks] = createSignal<Network[]>([]);
  createEffect(() => {
    getNetworks(p.deployment.serverID!).then(setNetworks);
  });

  const state = {
    editing,
    deployment,
    setDeployment,
    reset: load,
    networks,
  };

  return <context.Provider value={state}>{p.children}</context.Provider>;
};

export function useConfig() {
  return useContext(context) as State;
}
