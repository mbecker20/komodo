import { Deployment, Server } from "@monitor/types";
import { FastifyInstance } from "fastify";

export function toDashedName(name: string) {
  return name.toLowerCase().replaceAll(" ", "-");
}

export async function getDeploymentAndServer(
  app: FastifyInstance,
  deploymentID: string
) {
  const deployment = (await app.deployments.findById(
    deploymentID
  )) as Deployment;
  return {
    deployment,
    server: (await app.servers.findById(deployment.serverID)) as Server,
  };
}
