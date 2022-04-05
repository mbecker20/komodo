import { Deployment, User } from "@monitor/types";
import { deploymentChangelog, prettyStringify, UPDATE_DEPLOYMENT } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { remove } from "fs-extra";
import { DEPLOYMENT_REPO_PATH, PERMISSIONS_DENY_LOG } from "../../config";
import { deleteRepoPeriphery } from "../../util/periphery/git";
import { addDeploymentUpdate } from "../../util/updates";
import cloneRepo from "./clone";

async function updateDeployment(
  app: FastifyInstance,
  user: User,
  { deployment, note }: { deployment: Deployment; note?: string }
) {
  const preDeployment = await app.deployments.findById(deployment._id!);
  if (!preDeployment) return;
  if (user.permissions! < 2 && !preDeployment.owners.includes(user.username)) {
    addDeploymentUpdate(
      app,
      deployment._id!,
      UPDATE_DEPLOYMENT,
      "Update Deployment (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  app.deployActionStates.set(deployment._id!, "updating", true);
  app.broadcast(UPDATE_DEPLOYMENT, { deploymentID: deployment._id, complete: false });
  try {
    // this assumes no change to deployment name (ie cannot rename deployments after created)
    if (
      deployment.repo !== preDeployment.repo ||
      deployment.branch !== preDeployment.branch
    ) {
      if (deployment.repo) {
        console.log("cloning repo")
        await cloneRepo(app, user, deployment);
        console.log("repo cloned")
      } else {
        const server =
          deployment.serverID === app.core._id
            ? undefined
            : await app.servers.findById(deployment.serverID!);
        if (server) {
          await deleteRepoPeriphery(server, deployment);
        } else {
          await remove(DEPLOYMENT_REPO_PATH + deployment.containerName); // need to have this on periphery as well
        }
      }
    }
    console.log("setting owners undefined");
    // make sure owners cant be updated this way
    (deployment.owners as any) = undefined;
    console.log("updating")
    await app.deployments.updateById(deployment._id!, deployment);
    console.log("updated");
    addDeploymentUpdate(
      app,
      deployment._id!,
      UPDATE_DEPLOYMENT,
      "Update Deployment",
      {
        stdout: deploymentChangelog(preDeployment, deployment),
      },
      user.username,
      note
    );
    app.deployActionStates.set(deployment._id!, "updating", false);
    app.broadcast(UPDATE_DEPLOYMENT, {
      deploymentID: deployment._id,
      complete: true,
    });
    return deployment;
  } catch (error) {
    addDeploymentUpdate(
      app,
      deployment._id!,
      UPDATE_DEPLOYMENT,
      "Update Deployment (ERROR)",
      {
        stderr: prettyStringify(error),
      },
      user.username,
      note,
      true
    );
    app.deployActionStates.set(deployment._id!, "updating", false);
    app.broadcast(UPDATE_DEPLOYMENT, {
      deploymentID: deployment._id,
      complete: true,
    });
  }
}

export default updateDeployment;
