import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import builds from "./builds";
import deployments from "./deployments";
import accounts from "./accounts";
import listenerURL from "./listenerURL";
import networks from "./networks";
import servers from "./servers";
import updates from "./updates";
import secrets from "./secrets";
import listener from "./listener";

const routes = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app
    .register(updates)
    .register(builds)
    .register(deployments)
    .register(servers)
    .register(networks)
    .register(accounts)
    .register(secrets)
    .register(listenerURL)
    .register(listener);
	
	done();
});

export default routes;