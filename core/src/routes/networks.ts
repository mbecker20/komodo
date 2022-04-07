import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { getNetworks } from "@monitor/util";
import { getPeripheryNetworks } from "../util/periphery/networks";

const networks = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get(
    "/api/networks/:serverID",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { serverID } = req.params as { serverID: string };
      const server =
        serverID === app.core._id
          ? false
          : await app.servers.findById(serverID);
      if (server === undefined) {
        res.status(400);
        res.send("could not find server");
      }
      const networks = server
        ? await getPeripheryNetworks(server)
        : await getNetworks(app.dockerode);
      res.send(networks);
    }
  );
  done();
});

export default networks;
