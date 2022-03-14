import { Deployment, Server } from "@monitor/types";
import { FastifyInstance } from "fastify";

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
