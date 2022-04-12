import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import builds from "./builds";
import deployments from "./deployments";
import accounts from "./accounts";
import networks from "./networks";
import servers from "./servers";
import updates from "./updates";
import secrets from "./secrets";
import listener from "./listener";
import users from "./users";

const routes = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app
    .register(updates)
    .register(builds)
    .register(deployments)
    .register(servers)
    .register(networks)
    .register(accounts)
    .register(secrets)
    .register(listener)
    .register(users);
	
	done();
});

export default routes;