import axios from "axios";
import fileDownload from "js-file-download";
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
import {
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
} from "./client_types";
import { generateQuery, QueryObject } from "./helpers";

export class Client {
  loginOptions: LoginOptions | undefined;

  constructor(private baseURL: string, public token: string | null) {}

  async initialize() {
    this.loginOptions = await this.get_login_options();
    const params = new URLSearchParams(location.search);
    const exchange_token = params.get("token");
    if (exchange_token) {
      history.replaceState({}, "", MONITOR_BASE_URL);
      try {
        const jwt = await this.exchange_for_jwt(exchange_token);
        this.token = jwt;
        localStorage.setItem("access_token", jwt);
      } catch (error) {
        console.warn(error);
      }
    }
  }

  get_login_options(): Promise<LoginOptions> {
    return this.get("/auth/options");
  }

  login_with_github() {
    location.replace(`${MONITOR_BASE_URL}/auth/github/login`);
  }

  login_with_google() {
    location.replace(`${MONITOR_BASE_URL}/auth/google/login`);
  }

  async login(credentials: UserCredentials) {
    const jwt: string = await this.post("/auth/local/login", credentials);
    this.token = jwt;
    localStorage.setItem("access_token", this.token);
    return await this.get_user();
  }

  async signup(credentials: UserCredentials) {
    const jwt: string = await this.post("/auth/local/create_user", credentials);
    this.token = jwt;
    localStorage.setItem("access_token", this.token);
    return await this.get_user();
  }

  logout() {
    localStorage.removeItem("access_token");
    this.token = null;
  }

  async get_user(): Promise<User | false> {
    if (this.token) {
      try {
        return await this.get("/api/user");
      } catch (error: any) {
        this.logout();
        return false;
      }
    } else {
      return false;
    }
  }

 get_username(user_id: string): Promise<string> {
    return this.get(`/api/username/${user_id}`);
  }

  list_users(): Promise<User[]> {
    return this.get("/api/users");
  }

  exchange_for_jwt(exchange_token: string): Promise<string> {
    return this.post("/auth/exchange", { token: exchange_token });
  }

  get_github_webhook_base_url(): Promise<string> {
    return this.get("/api/github_webhook_base_url")
  }

  // deployment

  list_deployments(
    query?: QueryObject
  ): Promise<DeploymentWithContainerState[]> {
    return this.get("/api/deployment/list" + generateQuery(query));
  }

  get_deployment(id: string): Promise<DeploymentWithContainerState> {
    return this.get(`/api/deployment/${id}`);
  }

  get_deployment_action_state(id: string): Promise<DeploymentActionState> {
    return this.get(`/api/deployment/${id}/action_state`);
  }

  get_deployment_container_log(id: string, tail?: number): Promise<Log> {
    return this.get(`/api/deployment/${id}/log${generateQuery({ tail })}`);
  }

  get_deployment_container_stats(id: string): Promise<DockerContainerStats> {
    return this.get(`/api/deployment/${id}/stats`);
  }

  get_deployment_deployed_version(id: string): Promise<string> {
    return this.get(`/api/deployment/${id}/deployed_version`);
  }

  create_deployment(body: CreateDeploymentBody): Promise<Deployment> {
    return this.post("/api/deployment/create", body);
  }

  create_full_deployment(deployment: Deployment): Promise<Deployment> {
    return this.post("/api/deployment/create_full", deployment);
  }

  copy_deployment(
    target_id: string,
    body: CopyDeploymentBody
  ): Promise<Deployment> {
    return this.post(`/api/deployment/${target_id}/copy`, body);
  }

  delete_deployment(id: string): Promise<Deployment> {
    return this.delete(`/api/deployment/${id}/delete`);
  }

  update_deployment(deployment: Deployment): Promise<Deployment> {
    return this.patch("/api/deployment/update", deployment);
  }

  reclone_deployment(deployment_id: string): Promise<Update> {
    return this.post(`/api/deployment/${deployment_id}/reclone`);
  }

  pull_deployment(deployment_id: string): Promise<Update> {
    return this.post(`/api/deployment/${deployment_id}/pull`);
  }

  deploy_container(deployment_id: string): Promise<Update> {
    return this.post(`/api/deployment/${deployment_id}/deploy`);
  }

  start_container(deployment_id: string): Promise<Update> {
    return this.post(`/api/deployment/${deployment_id}/start_container`);
  }

  stop_container(deployment_id: string): Promise<Update> {
    return this.post(`/api/deployment/${deployment_id}/stop_container`);
  }

  remove_container(deployment_id: string): Promise<Update> {
    return this.post(`/api/deployment/${deployment_id}/remove_container`);
  }

  async download_container_log(
    id: string,
    name: string,
    error?: boolean | undefined
  ) {
    const log = await this.get_deployment_container_log(id, 5000);
    const date = new Date();
    fileDownload(
      (error ? log.stderr : log.stdout) || "no log",
      `${name}-${error ? "error-" : ""}log-${date
        .toLocaleDateString()
        .replaceAll("/", "-")}.txt`
    );
  }

  // server

  list_servers(query?: QueryObject): Promise<ServerWithStatus[]> {
    return this.get("/api/server/list" + generateQuery(query));
  }

  get_server(server_id: string): Promise<ServerWithStatus> {
    return this.get(`/api/server/${server_id}`);
  }

  get_server_action_state(id: string): Promise<ServerActionState> {
    return this.get(`/api/server/${id}/action_state`);
  }

  get_server_github_accounts(id: string): Promise<string[]> {
    return this.get(`/api/server/${id}/github_accounts`);
  }

  get_server_docker_accounts(id: string): Promise<string[]> {
    return this.get(`/api/server/${id}/docker_accounts`);
  }

  get_server_version(id: string): Promise<string> {
    return this.get(`/api/server/${id}/version`);
  }

  get_server_system_info(id: string): Promise<SystemInformation> {
    return this.get(`/api/server/${id}/system_information`);
  }

  create_server(body: CreateServerBody): Promise<Server> {
    return this.post("/api/server/create", body);
  }

  create_full_server(server: Server): Promise<Server> {
    return this.post("/api/server/create_full", server);
  }

  delete_server(id: string): Promise<Server> {
    return this.delete(`/api/server/${id}/delete`);
  }

  update_server(server: Server): Promise<Server> {
    return this.patch("/api/server/update", server);
  }

  get_server_stats(
    server_id: string,
    query?: SystemStatsQuery
  ): Promise<SystemStats> {
    return this.get(
      `/api/server/${server_id}/stats${generateQuery(query as any)}`
    );
  }

  get_server_stats_history(
    server_id: string,
    query?: HistoricalStatsQuery
  ): Promise<SystemStatsRecord[]> {
    return this.get(
      `/api/server/${server_id}/stats/history${generateQuery(query as any)}`
    );
  }

  get_server_stats_at_ts(
    server_id: string,
    ts: number
  ): Promise<SystemStatsRecord> {
    return this.get(`/api/server/${server_id}/stats/at_ts?ts=${ts}`);
  }

  get_docker_networks(server_id: string): Promise<any[]> {
    return this.get(`/api/server/${server_id}/networks`);
  }

  prune_docker_networks(server_id: string): Promise<Log> {
    return this.post(`/api/server/${server_id}/networks/prune`);
  }

  get_docker_images(server_id: string): Promise<any[]> {
    return this.get(`/api/server/${server_id}/images`);
  }

  prune_docker_images(server_id: string): Promise<Log> {
    return this.post(`/api/server/${server_id}/images/prune`);
  }

  get_docker_containers(server_id: string): Promise<BasicContainerInfo[]> {
    return this.get(`/api/server/${server_id}/containers`);
  }

  prune_docker_containers(server_id: string): Promise<Log> {
    return this.post(`/api/server/${server_id}/containers/prune`);
  }

  // build

  list_builds(query?: QueryObject): Promise<Build[]> {
    return this.get("/api/build/list" + generateQuery(query));
  }

  get_build(build_id: string): Promise<Build> {
    return this.get(`/api/build/${build_id}`);
  }

  get_build_action_state(id: string): Promise<BuildActionState> {
    return this.get(`/api/build/${id}/action_state`);
  }

  get_build_versions(
    id: string,
    query?: BuildVersionsQuery
  ): Promise<BuildVersionsReponse> {
    return this.get(`/api/build/${id}/versions${generateQuery(query as any)}`);
  }

  create_build(body: CreateBuildBody): Promise<Build> {
    return this.post("/api/build/create", body);
  }

  create_full_build(build: Build): Promise<Build> {
    return this.post("/api/build/create_full", build);
  }

  copy_build(target_id: string, body: CopyBuildBody): Promise<Build> {
    return this.post(`/api/build/${target_id}/copy`, body);
  }

  delete_build(id: string): Promise<Build> {
    return this.delete(`/api/build/${id}/delete`);
  }

  update_build(build: Build): Promise<Build> {
    return this.patch("/api/build/update", build);
  }

  build(build_id: string): Promise<Update> {
    return this.post(`/api/build/${build_id}/build`);
  }

  reclone_build(id: string): Promise<Update> {
    return this.post(`/api/build/${id}/reclone`);
  }

  get_aws_builder_defaults(): Promise<AwsBuilderConfig> {
    return this.get("/api/build/aws_builder_defaults");
  }

  get_docker_organizations(): Promise<string[]> {
    return this.get("/api/build/docker_organizations");
  }

  // procedure

  list_procedures(query?: QueryObject): Promise<Procedure[]> {
    return this.get("/api/procedure/list" + generateQuery(query));
  }

  get_procedure(procedure_id: string): Promise<Procedure> {
    return this.get(`/api/procedure/${procedure_id}`);
  }

  create_procedure(body: CreateProcedureBody): Promise<Procedure> {
    return this.post("/api/procedure/create", body);
  }

  create_full_procedure(procedure: Procedure): Promise<Procedure> {
    return this.post("/api/procedure/create_full", procedure);
  }

  delete_procedure(id: string): Promise<Procedure> {
    return this.delete(`/api/procedure/${id}/delete`);
  }

  update_procedure(procedure: Procedure): Promise<Procedure> {
    return this.patch("/api/procedure/update", procedure);
  }

  run_procedure(id: string): Promise<Update> {
    return this.post(`/api/procedure/${id}/run`);
  }

  // group

  list_groups(query?: QueryObject): Promise<Group[]> {
    return this.get("/api/group/list" + generateQuery(query));
  }

  get_group(group_id: string): Promise<Group> {
    return this.get(`/api/group/${group_id}`);
  }

  create_group(body: CreateGroupBody): Promise<Group> {
    return this.post("/api/group/create", body);
  }

  create_full_group(group: Group): Promise<Group> {
    return this.post("/api/group/create_full", group);
  }

  delete_group(id: string): Promise<Group> {
    return this.delete(`/api/group/${id}/delete`);
  }

  update_group(group: Group): Promise<Group> {
    return this.patch("/api/group/update", group);
  }

  // updates
  // show_builds is only relevant for Deployment targets, must pass show_builds = true to include build updates of attached build_id
  list_updates(
    offset: number,
    target?: UpdateTarget,
    show_builds?: boolean,
    operations?: Operation[]
  ): Promise<Update[]> {
    return this.get(
      `/api/update/list${generateQuery({
        offset,
        type: target && target.type,
        id: target && target.id,
        show_builds,
        operations: operations?.join(","),
      })}`
    );
  }

  // api secrets

  create_api_secret(body: CreateSecretBody): Promise<string> {
    return this.post("/api/secret/create", body);
  }

  delete_api_secret(name: string): Promise<undefined> {
    return this.delete(`/api/secret/delete/${name}`);
  }

  // permissions

  update_user_permissions_on_target(
    body: PermissionsUpdateBody
  ): Promise<Update> {
    return this.post("/api/permissions/update", body);
  }

  modify_user_enabled(body: ModifyUserEnabledBody): Promise<Update> {
    return this.post("/api/permissions/modify_enabled", body);
  }

  modify_user_create_server_permissions(
    body: ModifyUserCreateServerBody
  ): Promise<Update> {
    return this.post("/api/permissions/modify_create_server", body);
  }

  modify_user_create_build_permissions(
    body: ModifyUserCreateBuildBody
  ): Promise<Update> {
    return this.post("/api/permissions/modify_create_build", body);
  }

  get<R = any>(url: string): Promise<R> {
    return axios({
      method: "get",
      url: this.baseURL + url,
      headers: {
        authorization: this.token ? `Bearer ${this.token}` : undefined,
      },
    }).then(({ data }) => data);
  }

  post<B = any, R = any>(url: string, body?: B): Promise<R> {
    return axios({
      method: "post",
      url: this.baseURL + url,
      headers: {
        authorization: `Bearer ${this.token}`,
      },
      data: body,
    }).then(({ data }) => data);
  }

  patch<B = any, R = any>(url: string, body: B): Promise<R> {
    return axios({
      method: "patch",
      url: this.baseURL + url,
      headers: {
        authorization: `Bearer ${this.token}`,
      },
      data: body,
    }).then(({ data }) => data);
  }

  delete<R = any>(url: string): Promise<R> {
    return axios({
      method: "delete",
      url: this.baseURL + url,
      headers: {
        authorization: `Bearer ${this.token}`,
      },
    }).then(({ data }) => data);
  }
}
