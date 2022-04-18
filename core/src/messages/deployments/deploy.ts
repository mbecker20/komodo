import { User } from "@monitor/types";
import {
  DEPLOY,
  prettyStringify,
} from "@monitor/util";
import { deleteContainer, dockerRun } from "@monitor/util-node";
import { FastifyInstance } from "fastify";
import { join } from "path";
import { WebSocket } from "ws";
import {
  PERMISSIONS_DENY_LOG,
  SECRETS,
  SYSROOT,
  SYS_DEPLOYMENT_REPO_PATH,
} from "../../config";
import { DEPLOYING } from "../../plugins/actionStates";
import { sendAlert } from "../../util/helpers";
import { deletePeripheryContainer } from "../../util/periphery/container";
import { deployPeriphery } from "../../util/periphery/deploy";
import { addDeploymentUpdate } from "../../util/updates";

async function deployDeployment(
  app: FastifyInstance,
  client: WebSocket,
  user: User,
  { deploymentID, note }: { deploymentID: string; note?: string }
) {
  if (app.deployActionStates.busy(deploymentID)) {
    sendAlert(client, "bad", "deployment busy, try again in a bit");
    return;
  }
  const deployment = await app.deployments.findById(deploymentID);
  if (!deployment) return;
  if (user.permissions! < 2 && !deployment.owners.includes(user.username)) {
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
    const server =
      deployment.serverID === app.core._id
        ? undefined
        : await app.servers.findById(deployment.serverID!);
    if (server) {
      // delete the container on periphery
      await deletePeripheryContainer(server, deployment.containerName!);
    } else {
      // delete the container on core
      await deleteContainer(deployment.containerName!);
    }
    const build = deployment.buildID
      ? await app.builds.findById(deployment.buildID)
      : undefined;
    const image =
      build && build.dockerBuildArgs
        ? join(build.dockerAccount || "", build.pullName!)
        : undefined;
    const { command, log, isError } = server
      ? await deployPeriphery(
          server,
          deployment,
          image,
          build && build.dockerAccount
        )
      : await dockerRun(
          {
            ...deployment,
            image: image ? image : deployment.image,
          },
          SYSROOT,
          deployment.repo && deployment.containerMount
            ? {
                repoFolder: join(
                  SYS_DEPLOYMENT_REPO_PATH,
                  deployment.containerName!,
                  deployment.repoMount || ""
                ),
                containerMount: deployment.containerMount,
              }
            : undefined,
          (build && build.dockerAccount) || deployment.dockerAccount,
          ((build && build.dockerAccount) || deployment.dockerAccount) &&
            SECRETS.DOCKER_ACCOUNTS[
              ((build && build.dockerAccount) || deployment.dockerAccount)!
            ]
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
      { stderr: prettyStringify(error) },
      user.username,
      note,
      true
    );
  }
  app.broadcast(DEPLOY, { complete: true, deploymentID });
  app.deployActionStates.set(deploymentID, DEPLOYING, false);
}

export default deployDeployment;
