import { Build, Deployment } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { HOST } from "../config";

export function toDashedName(name: string) {
  return name.toLowerCase().replaceAll(" ", "-");
}

export async function getBuildGithubListenerURL(
  app: FastifyInstance,
  buildID: string
) {
  const build = (await app.builds.findById(buildID, "pullName")) as Build;
  return `${HOST}/githubListener?pullName=${build.pullName}`;
}

export async function getDeploymentGithubListenerURL(
  app: FastifyInstance,
  deploymentID: string
) {
  const deployment = (await app.deployments.findById(
    deploymentID,
    "containerName"
  )) as Deployment;
  return `${HOST}/githubListener?containerName=${deployment.containerName}`;
}
