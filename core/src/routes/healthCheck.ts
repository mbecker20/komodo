import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const healthCheck = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/api/health-check", (_, res) => {
		res.send();
	})

	done();
});

export default healthCheck;