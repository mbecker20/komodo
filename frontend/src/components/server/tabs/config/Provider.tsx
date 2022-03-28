import { Network, Server, Update } from "@monitor/types";
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
import {
  ADD_UPDATE,
  CREATE_NETWORK,
  DELETE_NETWORK,
  PRUNE_NETWORKS,
  UPDATE_SERVER,
} from "../../../../state/actions";
import { useAppState } from "../../../../state/StateProvider";
import { getNetworks, getServer } from "../../../../util/query";

type ConfigServer = Server & { loaded: boolean; updated: boolean };

type State = {
  server: DeepReadonly<ConfigServer>;
  setServer: SetStoreFunction<ConfigServer>;
  reset: () => void;
  save: () => void;
  networks: Accessor<Network[]>;
};

const context = createContext<State>();

export const ConfigProvider: Component<{ server: Server }> = (p) => {
  const { ws } = useAppState();
  const [server, set] = createStore({
    ...p.server,
    loaded: false,
    updated: false,
  });

  const setServer = (...args: any) => {
    // @ts-ignore
    set(...args);
    set("updated", true);
  };

  const load = () => {
    console.log("load server");
    getServer(p.server._id!).then((server) => {
      set({
        ...server,
        isCore: server.isCore,
        loaded: true,
        updated: false,
      });
    });
  };
  createEffect(load);

  const [networks, setNetworks] = createSignal<Network[]>([]);
  const loadNetworks = () => {
    console.log("load networks");
    getNetworks(p.server._id!).then(setNetworks);
  };
  createEffect(loadNetworks);

  const save = () => {
    ws.send(UPDATE_SERVER, { server });
  };

  const unsub = ws.subscribe(
    [ADD_UPDATE],
    ({ update }: { update: Update }) => {
      if (update.serverID === p.server._id) {
        if (
          [CREATE_NETWORK, DELETE_NETWORK, PRUNE_NETWORKS].includes(
            update.operation
          )
        ) {
          loadNetworks()
        } else if ([UPDATE_SERVER].includes(update.operation)) {
          load();
        }
      }
    }
  );

  onCleanup(unsub);

  const state = {
    server,
    setServer,
    reset: load,
    save,
    networks,
  };
  return <context.Provider value={state}>{p.children}</context.Provider>;
};

export function useConfig() {
  return useContext(context) as State;
}
