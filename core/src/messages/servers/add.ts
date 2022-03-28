import { Server, User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { ADD_SERVER } from "@monitor/util";
import { addServerUpdate } from "../../util/updates";
import { serverStatusPeriphery } from "../../util/periphery/status";

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
	created.status = await serverStatusPeriphery(created) ? "OK" : "Could Not Be Reached";
	return created;
}

export default addServer