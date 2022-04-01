import {
  Accessor,
  Component,
  createContext,
  createResource,
  Resource,
  useContext,
} from "solid-js";
import { useLocalStorageToggle } from "../util/hooks";
import { getDockerAccounts, getGithubAccounts } from "../util/query";
import {
  useBuilds,
  useDeployments,
  useSelected,
  useServers,
  useUpdates,
} from "./hooks";
import socket from "./socket";
import { useUser } from "./UserProvider";

export type State = {
  servers: ReturnType<typeof useServers>;
  builds: ReturnType<typeof useBuilds>;
  deployments: ReturnType<typeof useDeployments>;
  updates: ReturnType<typeof useUpdates>;
  dockerAccounts: Resource<string[] | undefined>;
  githubAccounts: Resource<string[] | undefined>;
  sidebar: {
    open: Accessor<boolean>;
    toggle: () => void;
  };
};

const context = createContext<
  State & {
    ws: ReturnType<typeof socket>;
    selected: ReturnType<typeof useSelected>;
    logout: () => void;
  }
>();

export const AppStateProvider: Component<{}> = (p) => {
  const { user, permissions } = useUser();
  const [sidebarOpen, toggleSidebarOpen] = useLocalStorageToggle(
    "sidebar-open",
    true
  );
  const [dockerAccounts] = createResource(async () =>
    permissions() >= 1 ? getDockerAccounts() : undefined
  );
  const [githubAccounts] = createResource(async () =>
    permissions() >= 1 ? getGithubAccounts() : undefined
  );
  const state: State = {
    servers: useServers(),
    builds: useBuilds(),
    deployments: useDeployments(),
    updates: useUpdates(),
    dockerAccounts,
    githubAccounts,
    sidebar: {
      open: sidebarOpen,
      toggle: toggleSidebarOpen,
    },
  };

  // created prior state before, to pass state easily
  const selected = useSelected(state);
  const ws = socket(user(), state, selected);

  const { logout } = useUser();

  return (
    <context.Provider
      value={{
        ...state,
        ws,
        selected,
        logout: () => {
          logout();
          ws.close();
        },
      }}
    >
      {p.children}
    </context.Provider>
  );
};

export function useAppState() {
  return useContext(context) as State & {
    ws: ReturnType<typeof socket>;
    selected: ReturnType<typeof useSelected>;
    logout: () => void;
  };
}
