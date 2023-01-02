import { useNavigate } from "@solidjs/router";
import { createContext, ParentComponent, useContext } from "solid-js";
import { useWindowKeyDown } from "../util/hooks";
import {
  useBuilds,
  useDeployments,
  useServers,
  useServerStats,
  useUpdates,
  useUsernames,
} from "./hooks";
import connectToWs from "./ws";
import { useUser } from "./UserProvider";
import { PermissionLevel } from "../types";

export type State = {
  usernames: ReturnType<typeof useUsernames>
  servers: ReturnType<typeof useServers>;
  getPermissionOnServer: (id: string) => PermissionLevel;
  serverStats: ReturnType<typeof useServerStats>;
  builds: ReturnType<typeof useBuilds>;
  getPermissionOnBuild: (id: string) => PermissionLevel;
  deployments: ReturnType<typeof useDeployments>;
  getPermissionOnDeployment: (id: string) => PermissionLevel;
  updates: ReturnType<typeof useUpdates>;
};

const context = createContext<
  State & {
    ws: ReturnType<typeof connectToWs>;
    logout: () => void;
  }
>();

export const AppStateProvider: ParentComponent = (p) => {
  const { user, logout } = useUser();
  const navigate = useNavigate();
  const userId = (user()._id as any).$oid as string;
  const servers = useServers();
  const builds = useBuilds();
  const deployments = useDeployments();
  const usernames = useUsernames();
  const state: State = {
    usernames,
    servers,
    getPermissionOnServer: (id: string) => {
      const server = servers.get(id)!;
      const permissions = server.server.permissions![userId] as
        | PermissionLevel
        | undefined;
      if (permissions) {
        return permissions;
      } else {
        return PermissionLevel.None;
      }
    },
    builds,
    getPermissionOnBuild: (id: string) => {
      const build = builds.get(id)!;
      const permissions = build.permissions![userId] as
        | PermissionLevel
        | undefined;
      if (permissions) {
        return permissions;
      } else {
        return PermissionLevel.None;
      }
    },
    deployments,
    getPermissionOnDeployment: (id: string) => {
      const deployment = deployments.get(id)!;
      const permissions = deployment.deployment.permissions![userId] as
        | PermissionLevel
        | undefined;
      if (permissions) {
        return permissions;
      } else {
        return PermissionLevel.None;
      }
    },
    serverStats: useServerStats(),
    updates: useUpdates(),
  };

  // createEffect(() => {
  //   console.log("deployments", deployments.collection());
  // })

  // createEffect(() => {
  //   console.log("servers", servers.collection());
  // });

  // createEffect(() => {
  //   console.log("builds", builds.collection());
  // });

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
