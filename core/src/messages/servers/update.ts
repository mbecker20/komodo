import { Server, User } from "@monitor/types";
import { FastifyInstance } from "fastify";

async function updateServer(app: FastifyInstance, user: User, { server }: { server: Server }) {
	if (user.permissions! < 2) {
		return;
	}
	const preServer = await app.servers.findById(server._id!);
	await app.servers.updateById(server._id!, server);
	return server;
}

export default updateServer;