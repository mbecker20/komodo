import fastify from "fastify";
import fastifyCors from "fastify-cors";
import auth from "./auth";
import { LOG, PORT } from "./config";
import db from "./db";

const app = fastify({ logger: LOG })
	.register(fastifyCors, { origin: "*" })
  .register(db)
  .register(auth);

app.listen(PORT, (err, address) => {
  if (err) {
    app.log.error(err);
    process.exit(1);
  }
  console.log(`monitor core listening at ${address}`);
});
