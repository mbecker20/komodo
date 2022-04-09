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
  SERVER_OWNER_UPDATE,
  UPDATE_SERVER,
} from "../../../../state/actions";
import { useAppState } from "../../../../state/StateProvider";
import { useUser } from "../../../../state/UserProvider";
import { getNetworks, getServer } from "../../../../util/query";

type ConfigServer = Server & { loaded: boolean; updated: boolean };

type State = {
  server: DeepReadonly<ConfigServer>;
  setServer: SetStoreFunction<ConfigServer>;
  reset: () => void;
  save: () => void;
  networks: Accessor<Network[]>;
  userCanUpdate: () => boolean;
};

const context = createContext<State>();

export const ConfigProvider: Component<{}> = (p) => {
  const { ws, selected, servers } = useAppState();
  const { username, permissions } = useUser();
  const [server, set] = createStore({
    ...servers.get(selected.id())!,
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
    getServer(selected.id()).then((server) => {
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
    getNetworks(selected.id()).then(setNetworks);
  };
  createEffect(loadNetworks);

  const save = () => {
    ws.send(UPDATE_SERVER, { server });
  };

  onCleanup(
    ws.subscribe([ADD_UPDATE], ({ update }: { update: Update }) => {
      if (update.serverID === selected.id()) {
        if (
          [CREATE_NETWORK, DELETE_NETWORK, PRUNE_NETWORKS].includes(
            update.operation
          )
        ) {
          loadNetworks();
        } else if ([UPDATE_SERVER].includes(update.operation)) {
          load();
        }
      }
    })
  );

  onCleanup(
    ws.subscribe(
      [SERVER_OWNER_UPDATE],
      async ({ serverID }: { serverID: string }) => {
        if (serverID === selected.id()) {
          const server = await getServer(selected.id());
          set("owners", server.owners);
        }
      }
    )
  );

  const userCanUpdate = () => {
    if (permissions() > 1) {
      return true;
    } else if (permissions() > 0 && server.owners.includes(username()!)) {
      return true;
    } else {
      return false;
    }
  };

  const state = {
    server,
    setServer,
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
