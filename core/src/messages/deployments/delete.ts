import { User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { DELETE_DEPLOYMENT } from "@monitor/util";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { addDeploymentUpdate, addSystemUpdate } from "../../util/updates";

const deploymentViewFields = [
  "name",
  "image",
  "ports",
  "volumes",
  "environment",
  "network",
  "logToAWS",
  "useServerRoot",
  "restart",
  "autoDeploy",
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
    const stopLog = (deployment.image || deployment.buildID) && ""; // stop container, either locally or remote
    addSystemUpdate(
      app,
      DELETE_DEPLOYMENT,
      "Delete Deployment",
      {
        stdout:
          "Removed:\n\n" +
          deploymentViewFields
            .map((field) => {
              return `${field}: ${JSON.stringify(deployment[field])}\n`;
            })
            .reduce((prev, curr) => prev + curr) +
          (stopLog && `\n\nDocker Stop Log:\n\n${JSON.stringify(stopLog)}`),
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
      { stderr: JSON.stringify(error) },
      user.username,
      note,
      true
    );
  }
}

export default deleteDeployment;
