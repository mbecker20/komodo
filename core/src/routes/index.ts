import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import builds from "./builds";
import deployments from "./deployments";
import listenerURL from "./listenerURL";
import servers from "./servers";
import updates from "./updates";

const routes = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app
		.register(updates)
		.register(builds)
		.register(deployments)
		.register(servers)
		.register(listenerURL);
	
	done();
});

export default routes;