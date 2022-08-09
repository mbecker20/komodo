import { User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { REMOVE_SERVER } from "@monitor/util";
import { deletePeripheryContainer } from "../../util/periphery/container";
import { addSystemUpdate } from "../../util/updates";

async function removeServer(
  app: FastifyInstance,
  user: User,
  {
    serverID,
    note,
    deleteAllContainers,
  }: { serverID: string; note?: string; deleteAllContainers?: boolean }
) {
	// will also delete all deployments on the server
  if (user.permissions! < 2) {
    return;
  }
  const server = await app.servers.findByIdAndDelete(serverID);
	if (!server) return;
	const deployments = await app.deployments.find({ serverID });
  deployments.forEach((deployment) => {
    if (deleteAllContainers) {
      deletePeripheryContainer(server, deployment.containerName!);
    }
    app.deployments.findByIdAndDelete(deployment._id!);
  });
  app.serverActionStates.delete(serverID);
  app.statsIntervals.remove(serverID);
  addSystemUpdate(app, REMOVE_SERVER, "Remove Server", {}, user.username, note);
  return server;
}

export default removeServer;
