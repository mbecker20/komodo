import { User } from "@monitor/types";
import {
  prettyStringify,
  RECLONE_DEPLOYMENT_REPO,
} from "@monitor/util";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { sendAlert } from "../../util/helpers";
import { addDeploymentUpdate } from "../../util/updates";
import cloneRepo from "./clone";

async function recloneDeployment(
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
  if (!deployment) {
    sendAlert(client, "bad", "deployment not found");
    return;
  }
  if (user.permissions! < 2 && !deployment.owners.includes(user.username)) {
    addDeploymentUpdate(
      app,
      deploymentID,
      RECLONE_DEPLOYMENT_REPO,
      "Reclone Deployment (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  app.deployActionStates.set(deploymentID, "recloning", true);
  app.broadcast(
    RECLONE_DEPLOYMENT_REPO,
    {
      deploymentID,
      complete: false,
    },
    app.deploymentUserFilter(deploymentID)
  );
  try {
    // this assumes no change to deployment name (ie cannot rename deployments after created)
    if (deployment.repo) {
      await cloneRepo(app, user, deployment);
    } else {
      sendAlert(client, "bad", "deployment has no repo configured");
    }
  } catch (error) {
    addDeploymentUpdate(
      app,
      deploymentID,
      RECLONE_DEPLOYMENT_REPO,
      "Reclone Deployment (ERROR)",
      {
        stderr: prettyStringify(error),
      },
      user.username,
      note,
      true
    );
  }
  app.deployActionStates.set(deployment._id!, "recloning", false);
  app.broadcast(
    RECLONE_DEPLOYMENT_REPO,
    {
      deploymentID,
      complete: true,
    },
    app.deploymentUserFilter(deploymentID)
  );
}

export default recloneDeployment;
