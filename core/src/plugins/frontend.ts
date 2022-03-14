import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import fastifyStatic from "fastify-static";
import { resolve } from "path";
import { FRONTEND_PATH } from "../config";

const frontend = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.register(fastifyStatic, {
    root: resolve(FRONTEND_PATH),
  });
  done();
});

export default frontend;
