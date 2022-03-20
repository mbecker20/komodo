import { Component, createContext, useContext } from "solid-js";
import {
  useBuilds,
  useDeployments,
  useServers,
  useUpdates,
  useWs,
} from "./hooks";

export type State = {
  servers: ReturnType<typeof useServers>;
  builds: ReturnType<typeof useBuilds>;
  deployments: ReturnType<typeof useDeployments>;
  updates: ReturnType<typeof useUpdates>;
};

const context = createContext<State & { ws: ReturnType<typeof useWs> }>();

export const Provider: Component<{}> = (p) => {
  const state: State = {
    servers: useServers(),
    builds: useBuilds(),
    deployments: useDeployments(),
    updates: useUpdates(),
  };

  const ws = useWs(state);

  return (
    <context.Provider value={{ ...state, ws }}>{p.children}</context.Provider>
  );
};

export function useAppState() {
  return useContext(context) as State & { ws: ReturnType<typeof useWs> };
}