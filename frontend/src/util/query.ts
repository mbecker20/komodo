import { Builds, Deployments, Servers, Update } from "@monitor/types";
import { client } from "..";
import { generateQuery } from "./helpers";

export async function getUpdates(query?: {
  offset?: number;
  buildID?: string;
  serverID?: string;
  deploymentID?: string;
}) {
  return (await client.get("/api/updates" + generateQuery(query))) as Update[];
}

export async function getBuilds() {
  return (await client.get("/api/builds")) as Builds;
}

export async function getDeployments(query?: { serverID?: string }) {
  return (await client.get(
    "/api/deployments" + generateQuery(query)
  )) as Deployments;
}

export async function getServers() {
  return (await client.get("/api/servers")) as Servers;
}