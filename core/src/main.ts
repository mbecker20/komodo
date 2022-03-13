import fastify from "fastify";
import fastifyCors from "fastify-cors";
import { LOG, PORT } from "./config";

const app = fastify({ logger: LOG })
	.register(fastifyCors, { origin: "*" });

app.listen(PORT, (err, address) => {
  if (err) {
    app.log.error(err);
    process.exit(1);
  }
  console.log(`monitor core listening at ${address}`);
});
