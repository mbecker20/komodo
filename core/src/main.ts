import fastify from "fastify";
import fastifyCors from "fastify-cors";
import fastifyHelmet from "fastify-helmet";
import { LOG, PORT } from "./config";
import auth from "./plugins/auth";
import db from "./plugins/db";
import ws from "./plugins/ws";
import docker from "./plugins/docker";
import frontend from "./plugins/frontend";

const app = fastify({ logger: LOG })
  .register(fastifyCors)
  .register(fastifyHelmet)
  .register(db)
  .register(docker)
  .register(auth)
  .register(ws)
  .register(frontend);

app.listen(PORT, (err, address) => {
  if (err) {
    app.log.error(err);
    process.exit(1);
  }
  console.log(`monitor core listening at ${address}`);
});