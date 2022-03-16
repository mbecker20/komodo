import { User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { UPDATE_DEPLOYMENT } from ".";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { addDeploymentUpdate } from "../../util/updates";

async function updateDeployment(
  app: FastifyInstance,
  user: User,
  { deploymentID, note }: { deploymentID: string; note?: string }
) {
  const preDeployment = await app.deployments.findById(deploymentID);
	if (!preDeployment) return;
	if (user.permissions! < 2 && user.username !== preDeployment.owner) {
		addDeploymentUpdate(
      app,
      deploymentID,
      UPDATE_DEPLOYMENT,
      "Update Deployment (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
		return;
	}
	
}

export default updateDeployment;
