import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const status = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/status", { onRequest: [app.auth] }, (_, res) => {
		res.send("OK");
	});
	done();
});

export default status;