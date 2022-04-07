import { intoCollection } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { serverStatusPeriphery } from "../util/periphery/status";

const servers = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get("/api/servers", { onRequest: [app.auth, app.userEnabled] }, async (req, res) => {
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

  app.get("/api/server/:id", { onRequest: [app.auth, app.userEnabled] }, async (req, res) => {
    const { id } = req.params as { id: string };
    const server = await app.servers.findById(id);
    if (!server) {
      res.status(400);
      res.send("server not found");
      return;
    }
    server.status = (await serverStatusPeriphery(server))
      ? "OK"
      : "Could Not Be Reached";
    res.send(server);
  });

  app.get(
    "/api/server/:id/action-state",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const state = app.serverActionStates.getJSON(id);
      res.send(state);
    }
  );
  
  done();
});

export default servers;
