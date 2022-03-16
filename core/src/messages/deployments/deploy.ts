import { User } from "@monitor/types";
import { deleteContainer, dockerRun } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { DEPLOY } from ".";
import { PERMISSIONS_DENY_LOG, SYSROOT } from "../../config";
import { DEPLOYING } from "../../plugins/actionStates";
import { deletePeripheryContainer } from "../../util/periphery/container";
import { deployPeriphery } from "../../util/periphery/deploy";
import { addDeploymentUpdate } from "../../util/updates";

async function deployDeployment(
  app: FastifyInstance,
  user: User,
  { deploymentID, note }: { deploymentID: string; note?: string }
) {
  const deployment = await app.deployments.findById(deploymentID);
  if (!deployment) return;
  if (user.permissions! < 2 && user.username !== deployment.owner) {
    addDeploymentUpdate(
      app,
      deploymentID,
      DEPLOY,
      "Deploy (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  if (app.deployActionStates.get(deploymentID, DEPLOYING)) return;
  app.deployActionStates.set(deploymentID, DEPLOYING, true);
  app.broadcast(DEPLOY, { complete: false, deploymentID });
  try {
    const server = deployment.serverID
      ? await app.servers.findById(deployment.serverID)
      : undefined;
    if (server) {
      // delete the container on periphery
      await deletePeripheryContainer(server, deployment.containerName!);
    } else {
      // delete the container on core
      await deleteContainer(deployment.containerName!);
    }
    const { command, log, isError } = server
      ? await deployPeriphery(server, deployment)
      : await dockerRun(
          deployment,
          SYSROOT,
          deployment.repo && deployment.containerMount
            ? {
                repoFolder: SYSROOT + "/repos",
                containerMount: deployment.containerMount,
              }
            : undefined
        );
    addDeploymentUpdate(
      app,
      deploymentID,
      DEPLOY,
      command,
      log,
      user.username,
      note,
      isError
    );
  } catch (error) {
    addDeploymentUpdate(
      app,
      deploymentID,
      DEPLOY,
      "Deploy (ERROR)",
      { stderr: JSON.stringify(error) },
      user.username,
      note,
      true
    );
  }
  app.broadcast(DEPLOY, { complete: true, deploymentID });
  app.deployActionStates.set(deploymentID, DEPLOYING, false);
}

export default deployDeployment;