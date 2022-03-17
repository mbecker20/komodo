import { prune } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const server = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/prune", { onRequest: [app.auth] }, async (_, res) => {
		return await prune();
	});

	done();
});

export default server;