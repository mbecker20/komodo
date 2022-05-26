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
import healthCheck from "./healthCheck";
import pm2 from "./pm2";

const routes = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app
    .register(healthCheck)
    .register(updates)
    .register(builds)
    .register(deployments)
    .register(servers)
    .register(pm2)
    .register(networks)
    .register(accounts)
    .register(secrets)
    .register(listener)
    .register(users);

  done();
});

export default routes;
