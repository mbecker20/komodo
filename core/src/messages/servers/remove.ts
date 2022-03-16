import { User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { REMOVE_SERVER } from ".";
import { addSystemUpdate } from "../../util/updates";

async function removeServer(
  app: FastifyInstance,
  user: User,
  { serverID, note }: { serverID: string; note?: string }
) {
	if (user.permissions! < 2) {
		return;
	}
	const server = await app.servers.findByIdAndDelete(serverID);
	addSystemUpdate(
		app,
		REMOVE_SERVER,
		"Remove Server",
		{},
		user.username,
		note,
	);
	return true;
}

export default removeServer;
