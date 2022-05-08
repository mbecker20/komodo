import { getDockerStats, pruneImages } from "@monitor/util-node";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const server = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/images/prune", { onRequest: [app.auth] }, async (_, res) => {
		const log = await pruneImages();
		res.send(log);
	});

	app.get("/stats", { onRequest: [app.auth] }, async (req, res) => {
		const stats = await getDockerStats();
		res.send(stats);
	});

	done();
});

export default server;