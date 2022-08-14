import {
  Build,
  BuildActionState,
  Builds,
  CommandLogError,
  ContainerStatus,
  DeployActionState,
  Deployment,
  Deployments,
  DockerStat,
  Log,
  Network,
  PM2Process,
  Server,
  Servers,
  StoredStats,
  SystemStats,
  Update,
  User,
} from "@monitor/types";
import { client } from "..";
import { generateQuery } from "./helpers";
import fileDownload from "js-file-download";

export async function getUpdates(query?: {
  offset?: number;
  buildID?: string;
  serverID?: string;
  deploymentID?: string;
}) {
  return await client.get<Update[]>("/api/updates" + generateQuery(query));
}

export async function getBuilds() {
  return await client.get<Builds>("/api/builds");
}

export async function getBuild(buildID: string) {
  return await client.get<Build>(`/api/build/${buildID}`);
}

export async function getBuildActionState(buildID: string) {
  return await client.get<BuildActionState>(
    `/api/build/${buildID}/action-state`
  );
}

export async function addOwnerToBuild(buildID: string, username: string) {
  return await client.post(`/api/build/${buildID}/${username}`);
}

export async function removeOwnerFromBuild(buildID: string, username: string) {
  return await client.delete(`/api/build/${buildID}/${username}`);
}

export async function getDeployments() {
  return await client.get<Deployments>("/api/deployments");
}

export async function getDeployment(deploymentID: string) {
  return await client.get<Deployment>("/api/deployment/" + deploymentID);
}

export async function getDeploymentLog(deploymentID: string, tail?: number) {
  return await client.get<Log>(
    `/api/deployment/${deploymentID}/log${generateQuery({ tail })}`
  );
}

export async function downloadDeploymentLog(
  deploymentID: string,
  name: string,
  error?: boolean | undefined
) {
  const log = await client.get<Log>(
    `/api/deployment/${deploymentID}/log/download`
  );
  const date = new Date();
  fileDownload(
    (error ? log.stderr : log.stdout) || "no log",
    `${name}-${error ? "error-" : ""}log-${date
      .toLocaleDateString()
      .replaceAll("/", "-")}.txt`
  );
}

export async function getDeploymentStatus(deploymentID: string) {
  return await client.get<ContainerStatus | "not deployed">(
    `/api/deployment/${deploymentID}/status`
  );
}

export async function getDeploymentActionState(deploymentID: string) {
  return await client.get<DeployActionState>(
    `/api/deployment/${deploymentID}/action-state`
  );
}

export async function addOwnerToDeployment(
  deploymentID: string,
  username: string
) {
  return await client.post(`/api/deployment/${deploymentID}/${username}`);
}

export async function removeOwnerFromDeployment(
  deploymentID: string,
  username: string
) {
  return await client.delete(`/api/deployment/${deploymentID}/${username}`);
}

export async function getServers() {
  return await client.get<Servers>("/api/servers");
}

export async function getServer(id: string) {
  return await client.get<Server>(`/api/server/${id}`);
}

export async function getServerActionState(serverID: string) {
  return await client.get<DeployActionState>(
    `/api/server/${serverID}/action-state`
  );
}

export async function getServerStats(id: string) {
  return await client.get<DockerStat[]>(`/api/server/${id}/stats`);
}

export async function getServerSystemStats(id: string) {
  return await client.get<SystemStats>(`/api/server/${id}/sys-stats`);
}

export async function getServerStatsHistory(
  id: string,
  offset?: number,
  numPts?: number,
  skip?: number
) {
  return await client.get<StoredStats[]>(
    `/api/server/${id}/stats-history${generateQuery({ offset, numPts, skip })}`
  );
}

export async function getPm2Processes(id: string) {
  return await client.get<PM2Process[]>(`/api/server/${id}/pm2/processes`);
}

export async function getPm2Log(serverID: string, name: string, lines = 50) {
  return await client.get<CommandLogError>(
    `/api/server/${serverID}/pm2/log/${name}` + generateQuery({ lines })
  );
}

export async function startPm2Process(serverID: string, name: string) {
  return await client.get<CommandLogError>(
    `/api/server/${serverID}/pm2/start/${name}`
  );
}

export async function stopPm2Process(serverID: string, name: string) {
  return await client.get<CommandLogError>(
    `/api/server/${serverID}/pm2/stop/${name}`
  );
}

export async function restartPm2Process(serverID: string, name: string) {
  return await client.get<CommandLogError>(
    `/api/server/${serverID}/pm2/restart/${name}`
  );
}

export async function deletePm2Process(serverID: string, name: string) {
  return await client.get<CommandLogError>(
    `/api/server/${serverID}/pm2/delete/${name}`
  );
}

export async function addOwnerToServer(serverID: string, username: string) {
  return await client.post(`/api/server/${serverID}/${username}`);
}

export async function removeOwnerFromServer(
  serverID: string,
  username: string
) {
  return await client.delete(`/api/server/${serverID}/${username}`);
}

export async function getNetworks(serverID: string) {
  return await client.get<Network[]>("/api/networks/" + serverID);
}

export async function getGithubAccounts() {
  return await client.get<string[]>("/api/accounts/github");
}

export async function getDockerAccounts() {
  return await client.get<string[]>("/api/accounts/docker");
}

export async function getUsers(username?: string, onlyUsers?: boolean) {
  return await client.get<User[]>(
    "/api/users" +
      generateQuery({ username, onlyUsers: onlyUsers ? "true" : undefined })
  );
}

export async function updateUser(body: {
  userID: string;
  permissions?: number;
  enabled?: boolean;
}) {
  return await client.post("/api/user/update", body);
}

export async function deleteUser(id: string) {
  return await client.delete(`/api/user/${id}`);
}
