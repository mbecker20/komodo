import { User } from "@monitor/types";
import { deleteContainer, startContainer, stopContainer } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { DELETE_CONTAINER, START_CONTAINER, STOP_CONTAINER } from ".";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { deletePeripheryContainer, startPeripheryContainer, stopPeripheryContainer } from "../../util/periphery/container";
import { addDeploymentUpdate } from "../../util/updates";

export async function startDeploymentContainer(
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
      START_CONTAINER,
      "Start Container (DENIED)",
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
  const { command, log, isError } = server
    ? await startPeripheryContainer(server, deployment.containerName!)
    : await startContainer(deployment.containerName!);
	addDeploymentUpdate(
		app,
		deploymentID,
		START_CONTAINER,
		command,
		log,
		user.username,
		note,
		isError
	);
}

export async function stopDeploymentContainer(
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
      STOP_CONTAINER,
      "Stop Container (DENIED)",
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
  const { command, log, isError } = server
    ? await stopPeripheryContainer(server, deployment.containerName!)
    : await stopContainer(deployment.containerName!);
  addDeploymentUpdate(
    app,
    deploymentID,
    STOP_CONTAINER,
    command,
    log,
    user.username,
    note,
    isError
  );
}

export async function deleteDeploymentContainer(
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
      DELETE_CONTAINER,
      "Delete Container (DENIED)",
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
  const { command, log, isError } = server
    ? await deletePeripheryContainer(server, deployment.containerName!)
    : await deleteContainer(deployment.containerName!);
  addDeploymentUpdate(
    app,
    deploymentID,
    DELETE_CONTAINER,
    command,
    log,
    user.username,
    note,
    isError
  );
}