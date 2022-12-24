import { useNavigate } from "@solidjs/router";
import { createContext, ParentComponent, useContext } from "solid-js";
import { useWindowKeyDown } from "../util/hooks";
import {
  useBuilds,
  useDeployments,
  useServers,
  useServerStats,
  useUpdates,
} from "./hooks";
import connectToWs from "./ws";
import { useUser } from "./UserProvider";

export type State = {
  servers: ReturnType<typeof useServers>;
  serverStats: ReturnType<typeof useServerStats>;
  builds: ReturnType<typeof useBuilds>;
  deployments: ReturnType<typeof useDeployments>;
  updates: ReturnType<typeof useUpdates>;
};

const context = createContext<
  State & {
    ws: ReturnType<typeof connectToWs>;
    logout: () => void;
  }
>();

export const AppStateProvider: ParentComponent = (p) => {
  const { logout } = useUser();
  const navigate = useNavigate();
  const state: State = {
    servers: useServers(),
    serverStats: useServerStats(),
    builds: useBuilds(),
    deployments: useDeployments(),
    updates: useUpdates(),
  };

  const ws = connectToWs(state);

  useWindowKeyDown((e) => {
    if (e.key === "H" && e.shiftKey) {
      navigate("/");
    }
  });

  return (
    <context.Provider
      value={{
        ...state,
        ws,
        logout: () => {
          ws.close();
          logout();
        },
      }}
    >
      {p.children}
    </context.Provider>
  );
};

export function useAppState() {
  return useContext(context) as State & {
    ws: ReturnType<typeof connectToWs>;
    logout: () => void;
  };
}
