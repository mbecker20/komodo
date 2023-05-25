import axios from "axios";
import {
  BuildStatsQuery,
  BuildStatsResponse,
  BuildVersionsQuery,
  CopyBuildBody,
  CopyDeploymentBody,
  CreateBuildBody,
  CreateDeploymentBody,
  CreateGroupBody,
  CreateProcedureBody,
  CreateSecretBody,
  CreateServerBody,
  LoginOptions,
  ModifyUserCreateBuildBody,
  ModifyUserCreateServerBody,
  ModifyUserEnabledBody,
  PermissionsUpdateBody,
  StopContainerQuery,
  UpdateDescriptionBody,
} from "./client_types";
import { MONITOR_BASE_URL } from "..";
import {
  AwsBuilderConfig,
  BasicContainerInfo,
  Build,
  BuildActionState,
  BuildVersionsReponse,
  Deployment,
  DeploymentActionState,
  DeploymentWithContainerState,
  DockerContainerStats,
  Group,
  HistoricalStatsQuery,
  Log,
  Operation,
  Procedure,
  Server,
  ServerActionState,
  ServerWithStatus,
  SystemInformation,
  SystemStats,
  SystemStatsQuery,
  SystemStatsRecord,
  Update,
  UpdateTarget,
  User,
  UserCredentials,
} from "../types";
import fileDownload from "js-file-download";
import { QueryObject, generateQuery } from "./helpers";

export function monitor_client(base_url: string, token: string | null) {
  const state = {
    base_url,
    token: token as string | null,
    login_options: undefined as LoginOptions | undefined,
    monitor_title: undefined as string | undefined,
    secrets_cache: {} as Record<string, string[]>,
    github_accounts_cache: {} as Record<string, string[]>,
    docker_accounts_cache: {} as Record<string, string[]>,
    server_version_cache: {} as Record<string, string>,
  };

  const request = request_client(state);

  const client = {
    // ===========
    // AUTH / USER
    // ===========

    async initialize() {
      const [loginOptions, monitorTitle] = await Promise.all([
        client.get_login_options(),
        client.get_monitor_title(),
      ]);
      state.login_options = loginOptions;
      state.monitor_title = monitorTitle;
      document.title = monitorTitle;
      const params = new URLSearchParams(location.search);
      const exchange_token = params.get("token");
      if (exchange_token) {
        history.replaceState({}, "", MONITOR_BASE_URL);
        try {
          const jwt = await client.exchange_for_jwt(exchange_token);
          token = jwt;
          localStorage.setItem("access_token", jwt);
        } catch (error) {
          console.warn(error);
        }
      }
    },

    get_login_options(): Promise<LoginOptions> {
      return request.get("/auth/options");
    },

    async login(credentials: UserCredentials) {
      const jwt: string = await request.post("/auth/local/login", credentials);
      state.token = jwt;
      localStorage.setItem("access_token", state.token);
      return await client.get_user();
    },

    async signup(credentials: UserCredentials) {
      const jwt: string = await request.post(
        "/auth/local/create_user",
        credentials
      );
      state.token = jwt;
      localStorage.setItem("access_token", state.token);
      return await client.get_user();
    },

    logout() {
      localStorage.removeItem("access_token");
      state.token = null;
    },

    async get_user(): Promise<User | false> {
      if (state.token) {
        try {
          return await request.get("/api/user");
        } catch (error: any) {
          client.logout();
          return false;
        }
      } else {
        return false;
      }
    },

    get_username(user_id: string): Promise<string> {
      return request.get(`/api/username/${user_id}`);
    },

    // admin only
    list_users(): Promise<User[]> {
      return request.get("/api/users");
    },

    // admin only
    get_user_by_id(user_id: string): Promise<User> {
      return request.get(`/api/user/${user_id}`);
    },

    exchange_for_jwt(exchange_token: string): Promise<string> {
      return request.post("/auth/exchange", { token: exchange_token });
    },

    create_api_secret(body: CreateSecretBody): Promise<string> {
      return request.post("/api/secret/create", body);
    },

    delete_api_secret(name: string): Promise<undefined> {
      return request.delete(`/api/secret/delete/${name}`);
    },

    // ====
    // MISC
    // ====

    get_github_webhook_base_url(): Promise<string> {
      return request.get("/api/github_webhook_base_url");
    },

    update_description(body: UpdateDescriptionBody): Promise<undefined> {
      return request.post("/api/update_description", body);
    },

    get_monitor_title(): Promise<string> {
      return request.get("/api/title");
    },

    list_updates(
      offset: number,
      target?: UpdateTarget,
      show_builds?: boolean,
      operations?: Operation[]
    ): Promise<Update[]> {
      return request.get(
        `/api/update/list${generateQuery({
          offset,
          type: target && target.type,
          id: target && target.id,
          show_builds,
          operations: operations?.join(","),
        })}`
      );
    },

    // ==========
    // DEPLOYMENT
    // ==========

    list_deployments(
      query?: QueryObject
    ): Promise<DeploymentWithContainerState[]> {
      return request.get("/api/deployment/list" + generateQuery(query));
    },

    get_deployment(id: string): Promise<DeploymentWithContainerState> {
      return request.get(`/api/deployment/${id}`);
    },

    get_deployment_action_state(id: string): Promise<DeploymentActionState> {
      return request.get(`/api/deployment/${id}/action_state`);
    },

    get_deployment_container_log(id: string, tail?: number): Promise<Log> {
      return request.get(`/api/deployment/${id}/log${generateQuery({ tail })}`);
    },

    get_deployment_container_stats(id: string): Promise<DockerContainerStats> {
      return request.get(`/api/deployment/${id}/stats`);
    },

    get_deployment_deployed_version(id: string): Promise<string> {
      return request.get(`/api/deployment/${id}/deployed_version`);
    },

    create_deployment(body: CreateDeploymentBody): Promise<Deployment> {
      return request.post("/api/deployment/create", body);
    },

    create_full_deployment(deployment: Deployment): Promise<Deployment> {
      return request.post("/api/deployment/create_full", deployment);
    },

    copy_deployment(
      target_id: string,
      body: CopyDeploymentBody
    ): Promise<Deployment> {
      return request.post(`/api/deployment/${target_id}/copy`, body);
    },

    delete_deployment(id: string): Promise<Deployment> {
      return request.delete(`/api/deployment/${id}/delete`);
    },

    update_deployment(deployment: Deployment): Promise<Deployment> {
      return request.patch("/api/deployment/update", deployment);
    },

    rename_deployment(deployment_id: string, new_name: string) {
      return request.patch(`/api/deployment/${deployment_id}/rename`, {
        new_name,
      });
    },

    reclone_deployment(deployment_id: string): Promise<Update> {
      return request.post(`/api/deployment/${deployment_id}/reclone`);
    },

    pull_deployment(deployment_id: string): Promise<Update> {
      return request.post(`/api/deployment/${deployment_id}/pull`);
    },

    deploy_container(
      deployment_id: string,
      query?: StopContainerQuery
    ): Promise<Update> {
      return request.post(
        `/api/deployment/${deployment_id}/deploy${generateQuery(query as any)}`
      );
    },

    start_container(deployment_id: string): Promise<Update> {
      return request.post(`/api/deployment/${deployment_id}/start_container`);
    },

    stop_container(
      deployment_id: string,
      query?: StopContainerQuery
    ): Promise<Update> {
      return request.post(
        `/api/deployment/${deployment_id}/stop_container${generateQuery(
          query as any
        )}`
      );
    },

    remove_container(
      deployment_id: string,
      query?: StopContainerQuery
    ): Promise<Update> {
      return request.post(
        `/api/deployment/${deployment_id}/remove_container${generateQuery(
          query as any
        )}`
      );
    },

    async download_container_log(
      id: string,
      name: string,
      error?: boolean | undefined
    ) {
      const log = await client.get_deployment_container_log(id, 5000);
      const date = new Date();
      fileDownload(
        (error ? log.stderr : log.stdout) || "no log",
        `${name}-${error ? "error-" : ""}log-${date
          .toLocaleDateString()
          .replaceAll("/", "-")}.txt`
      );
    },

    // ======
    // SERVER
    // ======

    list_servers(query?: QueryObject): Promise<ServerWithStatus[]> {
      return request.get("/api/server/list" + generateQuery(query));
    },

    get_server(server_id: string): Promise<ServerWithStatus> {
      return request.get(`/api/server/${server_id}`);
    },

    get_server_action_state(id: string): Promise<ServerActionState> {
      return request.get(`/api/server/${id}/action_state`);
    },

    get_server_github_accounts(id: string): Promise<string[]> {
      return request.get(`/api/server/${id}/github_accounts`);
    },

    get_server_docker_accounts(id: string): Promise<string[]> {
      return request.get(`/api/server/${id}/docker_accounts`);
    },

    get_server_available_secrets(id: string): Promise<string[]> {
      return request.get(`/api/server/${id}/secrets`);
    },

    get_server_version(id: string): Promise<string> {
      return request.get(`/api/server/${id}/version`);
    },

    get_server_system_info(id: string): Promise<SystemInformation> {
      return request.get(`/api/server/${id}/system_information`);
    },

    create_server(body: CreateServerBody): Promise<Server> {
      return request.post("/api/server/create", body);
    },

    create_full_server(server: Server): Promise<Server> {
      return request.post("/api/server/create_full", server);
    },

    delete_server(id: string): Promise<Server> {
      return request.delete(`/api/server/${id}/delete`);
    },

    update_server(server: Server): Promise<Server> {
      return request.patch("/api/server/update", server);
    },

    get_server_stats(
      server_id: string,
      query?: SystemStatsQuery
    ): Promise<SystemStats> {
      return request.get(
        `/api/server/${server_id}/stats${generateQuery(query as any)}`
      );
    },

    get_server_stats_history(
      server_id: string,
      query?: HistoricalStatsQuery
    ): Promise<SystemStatsRecord[]> {
      return request.get(
        `/api/server/${server_id}/stats/history${generateQuery(query as any)}`
      );
    },

    get_server_stats_at_ts(
      server_id: string,
      ts: number
    ): Promise<SystemStatsRecord> {
      return request.get(`/api/server/${server_id}/stats/at_ts?ts=${ts}`);
    },

    get_docker_networks(server_id: string): Promise<any[]> {
      return request.get(`/api/server/${server_id}/networks`);
    },

    prune_docker_networks(server_id: string): Promise<Log> {
      return request.post(`/api/server/${server_id}/networks/prune`);
    },

    get_docker_images(server_id: string): Promise<
      {
        RepoTags: string[];
        RepoDigests: string[];
        Size: number;
        Created: number;
        Id: string;
      }[]
    > {
      return request.get(`/api/server/${server_id}/images`);
    },

    prune_docker_images(server_id: string): Promise<Log> {
      return request.post(`/api/server/${server_id}/images/prune`);
    },

    get_docker_containers(server_id: string): Promise<BasicContainerInfo[]> {
      return request.get(`/api/server/${server_id}/containers`);
    },

    prune_docker_containers(server_id: string): Promise<Log> {
      return request.post(`/api/server/${server_id}/containers/prune`);
    },

    // =====
    // BUILD
    // =====

    list_builds(query?: QueryObject): Promise<Build[]> {
      return request.get("/api/build/list" + generateQuery(query));
    },

    get_build(build_id: string): Promise<Build> {
      return request.get(`/api/build/${build_id}`);
    },

    get_build_action_state(id: string): Promise<BuildActionState> {
      return request.get(`/api/build/${id}/action_state`);
    },

    get_build_versions(
      id: string,
      query?: BuildVersionsQuery
    ): Promise<BuildVersionsReponse[]> {
      return request.get(
        `/api/build/${id}/versions${generateQuery(query as any)}`
      );
    },

    get_build_stats(query?: BuildStatsQuery): Promise<BuildStatsResponse> {
      return request.get(`/api/build/stats${generateQuery(query as any)}`);
    },

    create_build(body: CreateBuildBody): Promise<Build> {
      return request.post("/api/build/create", body);
    },

    create_full_build(build: Build): Promise<Build> {
      return request.post("/api/build/create_full", build);
    },

    copy_build(target_id: string, body: CopyBuildBody): Promise<Build> {
      return request.post(`/api/build/${target_id}/copy`, body);
    },

    delete_build(id: string): Promise<Build> {
      return request.delete(`/api/build/${id}/delete`);
    },

    update_build(build: Build): Promise<Build> {
      return request.patch("/api/build/update", build);
    },

    build(build_id: string): Promise<Update> {
      return request.post(`/api/build/${build_id}/build`);
    },

    reclone_build(id: string): Promise<Update> {
      return request.post(`/api/build/${id}/reclone`);
    },

    get_aws_builder_defaults(): Promise<AwsBuilderConfig> {
      return request.get("/api/build/aws_builder_defaults");
    },

    get_docker_organizations(): Promise<string[]> {
      return request.get("/api/build/docker_organizations");
    },

    // =========
    // PROCEDURE
    // =========

    list_procedures(query?: QueryObject): Promise<Procedure[]> {
      return request.get("/api/procedure/list" + generateQuery(query));
    },

    get_procedure(procedure_id: string): Promise<Procedure> {
      return request.get(`/api/procedure/${procedure_id}`);
    },

    create_procedure(body: CreateProcedureBody): Promise<Procedure> {
      return request.post("/api/procedure/create", body);
    },

    create_full_procedure(procedure: Procedure): Promise<Procedure> {
      return request.post("/api/procedure/create_full", procedure);
    },

    delete_procedure(id: string): Promise<Procedure> {
      return request.delete(`/api/procedure/${id}/delete`);
    },

    update_procedure(procedure: Procedure): Promise<Procedure> {
      return request.patch("/api/procedure/update", procedure);
    },

    run_procedure(id: string): Promise<Update> {
      return request.post(`/api/procedure/${id}/run`);
    },

    // =====
    // GROUP
    // =====

    list_groups(query?: QueryObject): Promise<Group[]> {
      return request.get("/api/group/list" + generateQuery(query));
    },

    get_group(group_id: string): Promise<Group> {
      return request.get(`/api/group/${group_id}`);
    },

    create_group(body: CreateGroupBody): Promise<Group> {
      return request.post("/api/group/create", body);
    },

    create_full_group(group: Group): Promise<Group> {
      return request.post("/api/group/create_full", group);
    },

    delete_group(id: string): Promise<Group> {
      return request.delete(`/api/group/${id}/delete`);
    },

    update_group(group: Group): Promise<Group> {
      return request.patch("/api/group/update", group);
    },

    // ===========
    // PERMISSIONS
    // ===========

    update_user_permissions_on_target(
      body: PermissionsUpdateBody
    ): Promise<Update> {
      return request.post("/api/permissions/update", body);
    },

    modify_user_enabled(body: ModifyUserEnabledBody): Promise<Update> {
      return request.post("/api/permissions/modify_enabled", body);
    },

    modify_user_create_server_permissions(
      body: ModifyUserCreateServerBody
    ): Promise<Update> {
      return request.post("/api/permissions/modify_create_server", body);
    },

    modify_user_create_build_permissions(
      body: ModifyUserCreateBuildBody
    ): Promise<Update> {
      return request.post("/api/permissions/modify_create_build", body);
    },
  };

  return client;
}

function request_client(state: { base_url: string; token: string | null }) {
  return {
    async get<R = any>(url: string): Promise<R> {
      return await axios({
        method: "get",
        url: state.base_url + url,
        headers: {
          authorization: state.token ? `Bearer ${state.token}` : undefined,
        },
      }).then(({ data }) => data);
    },
    async post<B = any, R = any>(url: string, body?: B): Promise<R> {
      return await axios({
        method: "post",
        url: state.base_url + url,
        headers: {
          authorization: `Bearer ${state.token}`,
        },
        data: body,
      }).then(({ data }) => data);
    },
    async patch<B = any, R = any>(url: string, body: B): Promise<R> {
      return await axios({
        method: "patch",
        url: state.base_url + url,
        headers: {
          authorization: `Bearer ${state.token}`,
        },
        data: body,
      }).then(({ data }) => data);
    },
    async delete<R = any>(url: string): Promise<R> {
      return await axios({
        method: "delete",
        url: state.base_url + url,
        headers: {
          authorization: `Bearer ${state.token}`,
        },
      }).then(({ data }) => data);
    },
  };
}

export function login_with_github() {
  location.replace(`${MONITOR_BASE_URL}/auth/github/login`);
}

export function login_with_google() {
  location.replace(`${MONITOR_BASE_URL}/auth/google/login`);
}
