import { User } from "@monitor/types";
import { prune } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { PRUNE_SERVER } from ".";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { prunePeriphery } from "../../util/periphery/server";
import { addServerUpdate } from "../../util/updates";

async function pruneServer(app: FastifyInstance, user: User, { serverID, note }: { serverID: string; note?: string }) {
	if (user.permissions! < 2) {
		addServerUpdate(
			app,
			serverID,
			PRUNE_SERVER,
			"Prune Server (DENIED)",
			PERMISSIONS_DENY_LOG,
			user.username,
			note,
			true,
		)
		return;
	}
	const server = await app.servers.findById(serverID);
	if (!server) return;
	const { command, log, isError } = server.isCore ? await prune() : await prunePeriphery(server);
	addServerUpdate(
		app,
		serverID,
		PRUNE_SERVER,
		command,
		log,
		user.username,
		note,
		isError
	)
}

export default pruneServer;