import { Server, User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { ADD_SERVER } from ".";
import { addServerUpdate } from "../../util/updates";

async function addServer(app: FastifyInstance, user: User, { server }: { server: Server }) {
	if (user.permissions! < 2) {
		return;
	}
	const created = await app.servers.create(server);
	addServerUpdate(
		app,
		created._id!,
		ADD_SERVER,
		"Add Server",
		{},
		user.username
	);
	return created;
}

export default addServer