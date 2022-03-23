import { Accessor, Component, createContext, useContext } from "solid-js";
import { createStore, SetStoreFunction } from "solid-js/store";
import { useLocalStorageToggle } from "../util/hooks";
import { useBuilds, useDeployments, useSelected, useServers, useUpdates } from "./hooks";
import socket from "./socket";

export type State = {
  servers: ReturnType<typeof useServers>;
  builds: ReturnType<typeof useBuilds>;
  deployments: ReturnType<typeof useDeployments>;
  updates: ReturnType<typeof useUpdates>;
  selected: ReturnType<typeof useSelected>;
  sidebar: {
    open: Accessor<boolean>;
    toggle: () => void;
  };
};

const context = createContext<State & { ws: ReturnType<typeof socket> }>();

export const AppStateProvider: Component<{}> = (p) => {
  const [sidebarOpen, toggleSidebarOpen] = useLocalStorageToggle(
    "sidebar-open",
    true
  );
  const state: State = {
    servers: useServers(),
    builds: useBuilds(),
    deployments: useDeployments(),
    updates: useUpdates(),
    selected: useSelected(),
    sidebar: {
      open: sidebarOpen,
      toggle: toggleSidebarOpen,
    },
  };

  // created state before attaching ws, to pass state easily
  const ws = socket(state);

  return (
    <context.Provider value={{ ...state, ws }}>{p.children}</context.Provider>
  );
};

export function useAppState() {
  return useContext(context) as State & { ws: ReturnType<typeof socket> };
}
