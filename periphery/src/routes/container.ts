import { allContainerStatus, getContainerStatus } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const container = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get("/containers", { onRequest: [app.auth] }, async (_, res) => {
    const containers = await allContainerStatus(app.dockerode);
    res.send(containers);
  });

  app.get("/container/:name", { onRequest: [app.auth] }, async (req, res) => {
    const name: string = (req.params as any).name;
    const container = await getContainerStatus(app.dockerode, name);
    return res.send(container);
  });

  done();
});

export default container;
