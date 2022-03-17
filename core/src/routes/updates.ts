import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const updates = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get("/updates", { onRequest: [app.auth] }, async (req, res) => {
    // serves the last 10 updates going back an optional offest
    const { offset, buildID, serverID, deploymentID } = req.query as {
      offset?: number;
      buildID?: number;
      serverID?: number;
      deploymentID?: number;
    };
    const updates = await app.updates.getMostRecent(10, { buildID, serverID, deploymentID }, offset);
		res.send(updates);
  });
  done();
});

export default updates;
