import fastify from "fastify";
import fastifyCors from "fastify-cors";
import auth from "./util/auth";
import { LOG, PORT } from "./config";
import db from "./util/db";
import ws from "./util/ws";

const app = fastify({ logger: LOG })
	.register(fastifyCors)
  .register(db)
  .register(auth)
  .register(ws);

app.listen(PORT, (err, address) => {
  if (err) {
    app.log.error(err);
    process.exit(1);
  }
  console.log(`monitor core listening at ${address}`);
});