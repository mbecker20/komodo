import { User } from "@monitor/types";
import {
  DELETE_CONTAINER,
  START_CONTAINER,
  STOP_CONTAINER,
} from "@monitor/util";
import { deleteContainer, startContainer, stopContainer } from "@monitor/util-node";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { sendAlert } from "../../util/helpers";
import {
  deletePeripheryContainer,
  startPeripheryContainer,
  stopPeripheryContainer,
} from "../../util/periphery/container";
import { addDeploymentUpdate } from "../../util/updates";

export async function startDeploymentContainer(
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
      START_CONTAINER,
      "Start Container (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  const server =
    deployment.serverID === app.core._id
      ? undefined
      : await app.servers.findById(deployment.serverID!);
  app.broadcast(START_CONTAINER, { complete: false, deploymentID });
  const { command, log, isError } = server
    ? await startPeripheryContainer(server, deployment.containerName!)
    : await startContainer(deployment.containerName!);
  app.broadcast(START_CONTAINER, { complete: true, deploymentID });
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
      STOP_CONTAINER,
      "Stop Container (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  const server =
    deployment.serverID === app.core._id
      ? undefined
      : await app.servers.findById(deployment.serverID!);
  app.broadcast(STOP_CONTAINER, { complete: false, deploymentID });
  const { command, log, isError } = server
    ? await stopPeripheryContainer(server, deployment.containerName!)
    : await stopContainer(deployment.containerName!);
  app.broadcast(STOP_CONTAINER, { complete: true, deploymentID });
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
      DELETE_CONTAINER,
      "Delete Container (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  const server =
    deployment.serverID === app.core._id
      ? undefined
      : await app.servers.findById(deployment.serverID!);
  app.broadcast(DELETE_CONTAINER, { complete: false, deploymentID });
  const { command, log, isError } = server
    ? await deletePeripheryContainer(server, deployment.containerName!)
    : await deleteContainer(deployment.containerName!);
  app.broadcast(DELETE_CONTAINER, { complete: true, deploymentID });
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
