import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const status = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/status", (_, res) => {
		res.status(200);
		res.send();
	})
	done();
});

export default status;