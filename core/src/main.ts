import fastify from "fastify";
import fastifyCors from "fastify-cors";
import fastifyHelmet from "fastify-helmet";
import { LOGGER, PORT } from "./config";
import auth from "./plugins/auth";
import db from "./plugins/db";
import ws from "./plugins/ws";
import docker from "./plugins/docker";
import frontend from "./plugins/frontend";
import githubListener from "./plugins/github-listener";
import actionStates from "./plugins/actionStates";
import routes from "./routes";

async function main() {
  const app = fastify({ logger: LOGGER })
    .register(fastifyCors)
    .register(fastifyHelmet)
    .register(db)
    .register(docker)
    .register(auth)
    .register(ws)
    .register(frontend)
    .register(githubListener)
    .register(actionStates)
    .register(routes);
    
  app.listen(PORT, "0.0.0.0", async (err, address) => {
    if (err) {
      app.log.error(err);
      process.exit(1);
    }
    if (!LOGGER) console.log(`monitor core listening at ${address}`);
  });
}

main();
