import { Deployment, User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { CREATE_DEPLOYMENT } from "@monitor/util";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { toDashedName } from "../../util/helpers";
import { addDeploymentUpdate, addSystemUpdate } from "../../util/updates";

async function createDeployment(
  app: FastifyInstance,
  user: User,
  { deployment }: { deployment: Deployment }
) {
  if (user.permissions! < 1) {
    addSystemUpdate(
      app,
      CREATE_DEPLOYMENT,
      "Create Deployment (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      "",
      true
    );
    return;
  }
  const created = await app.deployments.create({
    ...deployment,
    containerName: toDashedName(deployment.name),
    owners: [user.username],
  });
  app.deployActionStates.add(created._id!);
  addDeploymentUpdate(
    app,
    created._id!,
    CREATE_DEPLOYMENT,
    "Create Deployment",
    { stdout: "Deployment Created: " + deployment.name },
    user.username
  );
}

export default createDeployment;
