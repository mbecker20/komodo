import fastify from "fastify";
import fastifyCors from "fastify-cors";
import { LOG, PORT } from "./config";
import auth from "./plugins/auth";
import db from "./plugins/db";
import ws from "./plugins/ws";
import docker from "./plugins/docker";

const app = fastify({ logger: LOG })
  .register(fastifyCors)
  .register(db)
  .register(docker)
  .register(auth)
  .register(ws);

app.listen(PORT, (err, address) => {
  if (err) {
    app.log.error(err);
    process.exit(1);
  }
  console.log(`monitor core listening at ${address}`);
});