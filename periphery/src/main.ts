import fastify from "fastify";
import { LOG, PORT } from "./config";
import docker from "./plugins/docker";

const app = fastify({ logger: LOG })
	.register(docker);

app.listen(PORT, (err, address) => {
  if (err) {
    app.log.error(err);
    process.exit(1);
  }
  console.log(`monitor periphery listening at ${address}`);
});