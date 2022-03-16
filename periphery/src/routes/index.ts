import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import container from "./container";
import deploy from "./deploy";
import git from "./git";

const routes = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.register(git).register(deploy).register(container);

  done();
});

export default routes;
