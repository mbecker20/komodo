import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const deployments = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get("/deployments", { onRequest: [app.auth] }, async (req, res) => {
    // returns the periphery deployments on the given serverID
    // returns the core deployments if no serverID is specified
    const { serverID } = req.query as { serverID?: string };
    const deployments = await app.deployments.findCollection(
      serverID ? { serverID } : { serverID: app.core._id },
      "name serverID",
    );
    res.send(deployments);
  });

  app.get("/deployment/:id", { onRequest: [app.auth] }, async (req, res) => {
    const { id } = req.params as { id: string };
    const deployment = await app.deployments.findById(id);
    res.send(deployment);
  });
  done();
});

export default deployments;
