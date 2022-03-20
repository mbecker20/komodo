import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const deployments = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get("/deployments", { onRequest: [app.auth] }, async (req, res) => {
    // returns the periphery deployments on the given serverID
    // returns the core deployments if no serverID is specified
    const { serverID } = req.query as { serverID?: string };
    const deployments = app.deployments.findCollection(
      serverID ? { serverID } : { serverID: app.core._id! }
    );
    res.send(deployments);
  });
  done();
});

export default deployments;
