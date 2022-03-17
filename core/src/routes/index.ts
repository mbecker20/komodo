import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import updates from "./updates";

const routes = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.register(updates);
	
	done();
});

export default routes;