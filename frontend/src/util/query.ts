import {
  Builds,
  ContainerStatus,
  Deployment,
  Deployments,
  Log,
  Network,
  Servers,
  Update,
} from "@monitor/types";
import { client } from "..";
import { generateQuery } from "./helpers";

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

export async function getDeployments(query?: { serverID?: string }) {
  return await client.get<Deployments>(
    "/api/deployments" + generateQuery(query)
  );
}

export async function getDeployment(deploymentID: string) {
  return await client.get<Deployment>("/api/deployment/" + deploymentID);
}

export async function getDeploymentLog(deploymentID: string, tail?: number) {
  return await client.get<Log>(
    `/api/deployment/${deploymentID}/log${generateQuery({ tail })}`
  );
}

export async function getDeploymentStatus(deploymentID: string) {
  return await client.get<ContainerStatus | "not created">(
    `/api/deployment/${deploymentID}/status`
  );
}

export async function getServers() {
  return await client.get<Servers>("/api/servers");
}

export async function getNetworks(serverID: string) {
  return await client.get<Network[]>("/api/networks/" + serverID);
}
