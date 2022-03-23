import { intoCollection } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { serverStatusPeriphery } from "../util/periphery/status";

const servers = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get("/api/servers", { onRequest: [app.auth] }, async (req, res) => {
    const servers = await app.servers.find({});
    await Promise.all(
      servers.map(async (server) => {
        server.status = (await serverStatusPeriphery(server))
          ? "OK"
          : "Could Not Be Reached";
      })
    );
    res.send(intoCollection(servers));
  });
  done();
});

export default servers;
