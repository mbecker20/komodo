import { User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { DELETE_DEPLOYMENT, prettyStringify } from "@monitor/util";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { addDeploymentUpdate, addSystemUpdate } from "../../util/updates";
import { deletePeripheryContainer } from "../../util/periphery/container";
import { WebSocket } from "ws";
import { sendAlert } from "../../util/helpers";
import { deleteContainer } from "@monitor/util-node";

const deploymentViewFields = [
  "name",
  "image",
  "network",
  "restart",
  "ports",
  "volumes",
  "environment",
]; // the fields shown in the update log

async function deleteDeployment(
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
      DELETE_DEPLOYMENT,
      "Delete Deployment (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  app.deployActionStates.set(deploymentID, "fullDeleting", true);
  app.broadcast(DELETE_DEPLOYMENT, { deploymentID, complete: false });
  try {
    if (deployment.image || deployment.buildID) {
      const server =
        deployment.serverID === app.core._id
          ? undefined
          : await app.servers.findById(deployment.serverID!);
      if (server) {
        await deletePeripheryContainer(server, deployment.containerName!);
      } else {
        await deleteContainer(deployment.containerName!);
      }
    }
    await app.deployments.findByIdAndDelete(deploymentID);
    app.deployActionStates.delete(deploymentID);
    addSystemUpdate(
      app,
      DELETE_DEPLOYMENT,
      "Delete Deployment",
      {
        stdout:
          "Removed:\n\n" +
          deploymentViewFields
            .map((field) => {
              return `${field}: ${prettyStringify(deployment[field])}\n`;
            })
            .reduce((prev, curr) => prev + curr),
      },
      user.username,
      note
    );
    app.broadcast(DELETE_DEPLOYMENT, { deploymentID, complete: true });
    return true;
  } catch (error) {
    app.deployActionStates.set(deploymentID, "fullDeleting", false);
    app.broadcast(DELETE_DEPLOYMENT, { deploymentID, complete: true });
    addSystemUpdate(
      app,
      DELETE_DEPLOYMENT,
      "Delete Deployment (ERROR)",
      { stderr: prettyStringify(error) },
      user.username,
      note,
      true
    );
  }
}

export default deleteDeployment;
