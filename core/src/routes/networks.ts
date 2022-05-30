import { getNetworks } from "@monitor/util-node";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { getPeripheryNetworks } from "../util/periphery/networks";

const networks = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get(
    "/api/networks/:serverID",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { serverID } = req.params as { serverID: string };
      const server = await app.servers.findById(serverID);
      if (server === undefined) {
        res.status(400);
        res.send("could not find server");
        return;
      }
      const networks = server.isCore
        ? await getNetworks(app.dockerode)
        : await getPeripheryNetworks(server);
      res.send(networks);
    }
  );
  done();
});

export default networks;
