import { Builds, Deployments, Servers, Update } from "@monitor/types";
import { client } from "..";
import { generateQuery } from "./helpers";

export async function getUpdates(query?: {
  offset?: number;
  buildID?: string;
  serverID?: string;
  deploymentID?: string;
}) {
  return (await client.get("/updates" + generateQuery(query))) as Update[];
}

export async function getBuilds() {
  return (await client.get("/builds")) as Builds;
}

export async function getDeployments(query?: { serverID?: string }) {
  return (await client.get(
    "/deployments" + generateQuery(query)
  )) as Deployments;
}

export async function getServers() {
  return (await client.get("/servers")) as Servers;
}

export async function getContainerStatus() {}
