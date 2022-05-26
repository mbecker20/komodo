import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { deletePeripheryPm2, getPeripheryPm2Log, getPeripheryPm2Processes, restartPeripheryPm2, startPeripheryPm2, stopPeripheryPm2 } from "../util/periphery/pm2";

const pm2 = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/api/server/:id/pm2/processes", { onRequest: [app.auth, app.userEnabled] }, async (req, res) => {
		const { id } = req.params as { id: string };
		const server = await app.servers.findById(id);
		if (!server) {
			res.status(400);
			res.send("server not found");
			return;
		}
		const user = (await app.users.findById(req.user.id))!;
		if (user.permissions! < 1 && !server.owners.includes(user.username)) {
			res.status(403);
			res.send("inadequate permissions");
			return;
		}
		const processes = server.isCore ? [] : await getPeripheryPm2Processes(server);
		res.send(processes);
	});

	app.get("/api/server/:id/pm2/log/:name", { onRequest: [app.auth, app.userEnabled] }, async (req, res) => {
		const { id, name } = req.params as { id: string; name: string };
		const server = await app.servers.findById(id);
		if (!server) {
			res.status(400);
			res.send("server not found");
			return;
		}
		const user = (await app.users.findById(req.user.id))!;
		if (user.permissions! < 1 && !server.owners.includes(user.username)) {
			res.status(403);
			res.send("inadequate permissions");
			return;
		}
		if (server.isCore) {
			res.status(400);
			res.send("monitor core does not support pm2");
			return;
		}
		const log = await getPeripheryPm2Log(server, name);
		res.send(log);
	});

	app.get("/api/server/:id/pm2/start/:name", { onRequest: [app.auth, app.userEnabled] }, async (req, res) => {
		const { id, name } = req.params as { id: string; name: string };
		const server = await app.servers.findById(id);
		if (!server) {
			res.status(400);
			res.send("server not found");
			return;
		}
		const user = (await app.users.findById(req.user.id))!;
		if (user.permissions! < 1 && !server.owners.includes(user.username)) {
			res.status(403);
			res.send("inadequate permissions");
			return;
		}
		if (server.isCore) {
			res.status(400);
			res.send("monitor core does not support pm2");
			return;
		}
		const log = await startPeripheryPm2(server, name);
		res.send(log);
	});

	app.get("/api/server/:id/pm2/stop/:name", { onRequest: [app.auth, app.userEnabled] }, async (req, res) => {
		const { id, name } = req.params as { id: string; name: string };
		const server = await app.servers.findById(id);
		if (!server) {
			res.status(400);
			res.send("server not found");
			return;
		}
		const user = (await app.users.findById(req.user.id))!;
		if (user.permissions! < 1 && !server.owners.includes(user.username)) {
			res.status(403);
			res.send("inadequate permissions");
			return;
		}
		if (server.isCore) {
			res.status(400);
			res.send("monitor core does not support pm2");
			return;
		}
		const log = await stopPeripheryPm2(server, name);
		res.send(log);
	});

	app.get("/api/server/:id/pm2/restart/:name", { onRequest: [app.auth, app.userEnabled] }, async (req, res) => {
		const { id, name } = req.params as { id: string; name: string };
		const server = await app.servers.findById(id);
		if (!server) {
			res.status(400);
			res.send("server not found");
			return;
		}
		const user = (await app.users.findById(req.user.id))!;
		if (user.permissions! < 1 && !server.owners.includes(user.username)) {
			res.status(403);
			res.send("inadequate permissions");
			return;
		}
		if (server.isCore) {
			res.status(400);
			res.send("monitor core does not support pm2");
			return;
		}
		const log = await restartPeripheryPm2(server, name);
		res.send(log);
	});

	app.get("/api/server/:id/pm2/delete/:name", { onRequest: [app.auth, app.userEnabled] }, async (req, res) => {
		const { id, name } = req.params as { id: string; name: string };
		const server = await app.servers.findById(id);
		if (!server) {
			res.status(400);
			res.send("server not found");
			return;
		}
		const user = (await app.users.findById(req.user.id))!;
		if (user.permissions! < 1 && !server.owners.includes(user.username)) {
			res.status(403);
			res.send("inadequate permissions");
			return;
		}
		if (server.isCore) {
			res.status(400);
			res.send("monitor core does not support pm2");
			return;
		}
		const log = await deletePeripheryPm2(server, name);
		res.send(log);
	});

	done();
});

export default pm2;