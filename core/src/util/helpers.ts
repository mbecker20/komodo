import { FastifyInstance } from "fastify";
import { HOST } from "../config";

export function toDashedName(name: string) {
  return name.toLowerCase().replaceAll(" ", "-");
}

export async function getBuildGithubListenerURL(
  app: FastifyInstance,
  buildID: string
) {
  const build = await app.builds.findById(buildID, "pullName");
  return `${HOST}/githubListener?pullName=${build?.pullName}`;
}

export async function getDeploymentGithubListenerURL(
  app: FastifyInstance,
  deploymentID: string
) {
  const deployment = await app.deployments.findById(
    deploymentID,
    "containerName"
  );
  return `${HOST}/githubListener?containerName=${deployment?.containerName}`;
}
