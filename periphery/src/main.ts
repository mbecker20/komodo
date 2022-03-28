import fastify from "fastify";
import fastifyCors from "fastify-cors";
import fastifyHelmet from "fastify-helmet";
import { LOG, PORT } from "./config";
import docker from "./plugins/docker";
import auth from "./plugins/auth";
import routes from "./routes";

const app = fastify({ logger: LOG })
  .register(fastifyHelmet)
  .register(fastifyCors)
  .register(docker)
  .register(auth)
  .register(routes);

app.listen(PORT, "0.0.0.0", (err, address) => {
  if (err) {
    app.log.error(err);
    process.exit(1);
  }
  console.log(`monitor periphery listening at ${address}`);
});