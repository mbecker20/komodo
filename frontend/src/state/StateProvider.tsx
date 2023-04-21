import { useNavigate } from "@solidjs/router";
import { createContext, createResource, ParentComponent, Resource, useContext } from "solid-js";
import { useWindowKeyDown } from "../util/hooks";
import {
  useBuilds,
  useBuildStats,
  useDeployments,
  useGroups,
  useProcedures,
  useServerDockerAccounts,
  useServerGithubAccounts,
  useServerInfo,
  useServers,
  useServerSecrets,
  useServerStats,
  useUpdates,
  useUsernames,
} from "./hooks";
import connectToWs from "./ws";
import { useUser } from "./UserProvider";
import { AwsBuilderConfig, PermissionLevel, UpdateTarget } from "../types";
import { client } from "..";
import { BuildStatsResponse } from "../util/client_types";

export type State = {
  usernames: ReturnType<typeof useUsernames>;
  servers: ReturnType<typeof useServers>;
  getPermissionOnServer: (id: string) => PermissionLevel;
  serverStats: ReturnType<typeof useServerStats>;
  serverInfo: ReturnType<typeof useServerInfo>;
  serverDockerAccounts: ReturnType<typeof useServerDockerAccounts>;
  serverGithubAccounts: ReturnType<typeof useServerGithubAccounts>;
  serverSecrets: ReturnType<typeof useServerSecrets>;
  ungroupedServerIds: () => string[] | undefined;
  builds: ReturnType<typeof useBuilds>;
  getPermissionOnBuild: (id: string) => PermissionLevel;
  deployments: ReturnType<typeof useDeployments>;
  getPermissionOnDeployment: (id: string) => PermissionLevel;
  groups: ReturnType<typeof useGroups>;
  getPermissionOnGroup: (id: string) => PermissionLevel;
  procedures: ReturnType<typeof useProcedures>;
  getPermissionOnProcedure: (id: string) => PermissionLevel;
  updates: ReturnType<typeof useUpdates>;
  aws_builder_config: Resource<AwsBuilderConfig>;
  docker_organizations: Resource<string[]>;
  github_webhook_base_url: Resource<string>;
  name_from_update_target: (target: UpdateTarget) => string;
  build_stats: ReturnType<typeof useBuildStats>;
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
  const groups = useGroups();
  const procedures = useProcedures();
  const deployments = useDeployments();
  const usernames = useUsernames();
  const [aws_builder_config] = createResource(() => client.get_aws_builder_defaults());
  const [docker_organizations] = createResource(() => client.get_docker_organizations());
  const [github_webhook_base_url] = createResource(() => client.get_github_webhook_base_url());
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
    ungroupedServerIds: () => {
      const groups_collection = () => Object.entries(groups.collection()!);
      return servers.ids()?.filter(server_id => {
        for (const [_, group] of groups_collection()) {
          for (const group_server_id of group.servers) {
            if (server_id === group_server_id) {
              return false;
            } 
          }
        }
        return true;
      });
    },
    builds,
    build_stats: useBuildStats(),
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
    serverStats: useServerStats(servers),
    serverInfo: useServerInfo(servers),
    serverDockerAccounts: useServerDockerAccounts(servers),
    serverGithubAccounts: useServerGithubAccounts(servers),
    serverSecrets: useServerSecrets(servers),
    groups,
    getPermissionOnGroup: (id: string) => {
      const group = groups.get(id)!;
      const permissions = group.permissions![userId] as
        | PermissionLevel
        | undefined;
      if (permissions) {
        return permissions;
      } else {
        return PermissionLevel.None;
      }
    },
    procedures,
    getPermissionOnProcedure: (id: string) => {
      const procedure = procedures.get(id)!;
      const permissions = procedure.permissions![userId] as
        | PermissionLevel
        | undefined;
      if (permissions) {
        return permissions;
      } else {
        return PermissionLevel.None;
      }
    },
    updates: useUpdates(),
    aws_builder_config,
    docker_organizations,
    github_webhook_base_url,
    name_from_update_target: (target) => {
      if (target.type === "Deployment" && deployments) {
        return deployments.get(target.id!)?.deployment.name || "deleted";
      } else if (target.type === "Server" && servers) {
        return servers.get(target.id)?.server.name || "deleted";
      } else if (target.type === "Build" && builds) {
        return builds.get(target.id)?.name || "deleted";
      } else if (target.type === "Group" && groups) {
        return groups.get(target.id)?.name || "deleted";
      } else {
        return "unknown"
      }
    },
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
