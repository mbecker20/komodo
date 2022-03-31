import { User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { deleteContainer, DELETE_DEPLOYMENT, prettyStringify } from "@monitor/util";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { addDeploymentUpdate, addSystemUpdate } from "../../util/updates";
import { deletePeripheryContainer } from "../../util/periphery/container";

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
  user: User,
  { deploymentID, note }: { deploymentID: string; note?: string }
) {
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
    return true;
  } catch (error) {
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
