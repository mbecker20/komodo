import { useParams } from "@solidjs/router";
import {
  Accessor,
  createContext,
  createEffect,
  createSignal,
  onCleanup,
  ParentComponent,
  useContext,
} from "solid-js";
import { createStore, SetStoreFunction } from "solid-js/store";
import { client } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { useUser } from "../../../../state/UserProvider";
import { Server, Operation, PermissionLevel } from "../../../../types";
import { getId } from "../../../../util/helpers";

type ConfigServer = Server & { loaded: boolean; updated: boolean };

type State = {
  server: ConfigServer;
  setServer: SetStoreFunction<ConfigServer>;
  reset: () => void;
  save: () => void;
  networks: Accessor<any[]>;
  userCanUpdate: () => boolean;
};

const context = createContext<State>();

export const ConfigProvider: ParentComponent<{}> = (p) => {
  const { ws, servers } = useAppState();
  const { id } = useParams();
  const { user } = useUser();
  const [server, set] = createStore({
    ...servers.get(id)!.server,
    loaded: false,
    updated: false,
  });

  const setServer = (...args: any) => {
    // @ts-ignore
    set(...args);
    set("updated", true);
  };

  const load = () => {
    // console.log("load server");
    client.get_server(id).then((server) => {
      set({
        ...server.server,
        loaded: true,
        updated: false,
      });
    });
  };
  createEffect(load);

  const [networks, setNetworks] = createSignal<any[]>([]);
  const loadNetworks = () => {
    console.log("load networks");
    client.get_docker_networks(id).then(setNetworks);
  };
  createEffect(loadNetworks);

  const save = () => {
    client.update_server(server);
  };

  onCleanup(
    ws.subscribe([Operation.UpdateServer], (update) => {
      if (update.target.id === id) {
        load();
      }
    })
  );

  // onCleanup(
  //   ws.subscribe(
  //     [SERVER_OWNER_UPDATE],
  //     async ({ serverID }: { serverID: string }) => {
  //       if (serverID === selected.id()) {
  //         const server = await getServer(selected.id());
  //         set("owners", server.owners);
  //       }
  //     }
  //   )
  // );

  const userCanUpdate = () =>
    user().admin ||
    servers.get(id)!.server.permissions![getId(user())] === PermissionLevel.Update;

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
