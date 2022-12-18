import axios from "axios";
import {
  BasicContainerInfo,
  Build,
  Deployment,
  DeploymentWithContainer,
  Log,
  PermissionLevel,
  PermissionsTarget,
  Server,
  SystemStats,
  Update,
  User,
} from "../types";
import { generateQuery, QueryObject } from "./helpers";

export class Client {
  constructor(private baseURL: string, private token: string | null) {}

  async login(username: string, password: string) {
    const jwt: string = await this.post("/auth/local/login", {
      username,
      password,
    });
    this.token = jwt;
    localStorage.setItem("access_token", this.token);
    return await this.getUser();
  }

  async signup(username: string, password: string) {
    const jwt: string = await this.post("/auth/local/create_user", {
      username,
      password,
    });
    this.token = jwt;
    localStorage.setItem("access_token", this.token);
    return await this.getUser();
  }

  logout() {
    localStorage.removeItem("access_token");
    this.token = null;
  }

  async getUser(): Promise<User | false> {
    if (this.token) {
      try {
        return await this.get("/api/user");
      } catch {
        this.logout();
        return false;
      }
    } else {
      return false;
    }
  }

  // deployment

  list_deployments(query?: QueryObject): Promise<DeploymentWithContainer[]> {
    return this.get("/api/deployment/list" + generateQuery(query));
  }

  get_deployment(id: string): Promise<DeploymentWithContainer> {
    return this.get(`/api/deployment/${id}`);
  }

  create_deployment(name: string, server_id: string): Promise<Deployment> {
    return this.post("/api/deployment/create", { name, server_id });
  }

  create_full_deployment(deployment: Deployment): Promise<Deployment> {
    return this.post("/api/deployment/create_full", deployment);
  }

  delete_deployment(deployment_id: string): Promise<Deployment> {
    return this.delete(`/api/deployment/delete/${deployment_id}`);
  }

  update_deployment(deployment: Deployment): Promise<Deployment> {
    return this.patch("/api/deployment/update", deployment);
  }

  reclone_deployment(deployment_id: string): Promise<Update> {
    return this.post(`/api/deployment/${deployment_id}/reclone`);
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

  // server

  list_servers(query?: QueryObject): Promise<Server[]> {
    return this.get("/api/server/list" + generateQuery(query));
  }

  get_server(server_id: string): Promise<Server> {
    return this.get(`/api/server/${server_id}`);
  }

  create_server(name: string, address: string): Promise<Server> {
    return this.post("/api/server/create", { name, address });
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

  get_server_stats(server_id: string): Promise<SystemStats> {
    return this.get(`/api/server/${server_id}/stats`);
  }

  prune_docker_networks(server_id: string): Promise<Log> {
    return this.post(`/api/server/${server_id}/networks/prune`);
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

  create_build(name: string, server_id: string): Promise<Build> {
    return this.post("/api/build/create", { name, server_id });
  }

  create_full_build(build: Build): Promise<Build> {
    return this.post("/api/build/create_full", build);
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

  // api secrets

  create_api_secret(name: string, expires?: string): Promise<string> {
    return this.post("/api/secret/create", { name, expires });
  }

  delete_api_secret(name: string): Promise<undefined> {
    return this.delete(`/api/secret/delete/${name}`);
  }

  // permissions

  update_user_permissions_on_target(
    user_id: string,
    permission: PermissionLevel,
    target_type: PermissionsTarget,
    target_id: string
  ): Promise<string> {
    return this.post("/api/permissions/update", {
      user_id,
      permission,
      target_type,
      target_id,
    });
  }

  modify_user_enabled(user_id: string, enabled: boolean): Promise<undefined> {
    return this.post("/api/permissions/update", { user_id, enabled });
  }

  async get<R = any>(url: string): Promise<R> {
    return await axios({
      method: "get",
      url: this.baseURL + url,
      headers: {
        authorization: `Bearer ${this.token}`,
      },
    }).then(({ data }) => data);
  }

  async post<B = any, R = any>(url: string, body?: B): Promise<R> {
    return await axios({
      method: "post",
      url: this.baseURL + url,
      headers: {
        authorization: `Bearer ${this.token}`,
      },
      data: body,
    }).then(({ data }) => data);
  }

  async patch<B = any, R = any>(url: string, body: B): Promise<R> {
    return await axios({
      method: "patch",
      url: this.baseURL + url,
      headers: {
        authorization: `Bearer ${this.token}`,
      },
      data: body,
    }).then(({ data }) => data);
  }

  async delete<R = any>(url: string): Promise<R> {
    return await axios({
      method: "delete",
      url: this.baseURL + url,
      headers: {
        authorization: `Bearer ${this.token}`,
      },
    }).then(({ data }) => data);
  }
}
