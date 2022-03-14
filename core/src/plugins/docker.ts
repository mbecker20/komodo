import Dockerode from "dockerode";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

declare module "fastify" {
  interface FastifyInstance {
    dockerode: Dockerode;
  }
}

const docker = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.decorate("dockerode", new Dockerode());
  done();
});

export default docker;
