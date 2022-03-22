import { Component, createContext, useContext } from "solid-js";
import {
  useBuilds,
  useDeployments,
  useServers,
  useUpdates,
} from "./hooks";
import socket from "./socket";

export type State = {
  servers: ReturnType<typeof useServers>;
  builds: ReturnType<typeof useBuilds>;
  deployments: ReturnType<typeof useDeployments>;
  updates: ReturnType<typeof useUpdates>;
};

const context = createContext<State & { ws: ReturnType<typeof socket> }>();

export const AppStateProvider: Component<{}> = (p) => {
  const state: State = {
    servers: useServers(),
    builds: useBuilds(),
    deployments: useDeployments(),
    updates: useUpdates(),
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