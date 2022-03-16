import { User } from "@monitor/types";
import { dockerRun } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { DEPLOY } from ".";
import { PERMISSIONS_DENY_LOG, SYSROOT } from "../../config";
import { deletePeripheryContainer } from "../../util/periphery/container";
import { deployPeriphery } from "../../util/periphery/deploy";
import { addDeploymentUpdate } from "../../util/updates";

async function deployDeployment(
  app: FastifyInstance,
  user: User,
  { deploymentID, note }: { deploymentID: string; note?: string },
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
  const server = deployment.serverID
    ? await app.servers.findById(deployment.serverID)
    : undefined;
  if (server) {
    // delete the container on periphery
    await deletePeripheryContainer(server, deployment.containerName!);
  } else {

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
  )
}
