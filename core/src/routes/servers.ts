import { intoCollection, SERVER_OWNER_UPDATE } from "@monitor/util";
import { getDockerStatsJson, getSystemStats } from "@monitor/util-node";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import {
  getPeripheryDockerStats,
  getPeripherySystemStats,
} from "../util/periphery/server";
import { serverStatusPeriphery } from "../util/periphery/status";

const servers = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get(
    "/api/servers",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const user = await app.users.findById(req.user.id);
      if (!user) {
        res.status(403);
        res.send("user not found");
        return;
      }
      const servers = await app.servers.find(
        user.permissions! > 1 ? {} : { owners: user.username }
      );
      await Promise.all(
        servers.map(async (server) => {
          server.status = (await serverStatusPeriphery(server))
            ? "OK"
            : "Could Not Be Reached";
        })
      );
      res.send(intoCollection(servers));
    }
  );

  app.get(
    "/api/server/:id",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };

      const server = await app.servers.findById(id);
      if (!server) {
        res.status(400);
        res.send("server not found");
        return;
      }
      const user = await app.users.findById(req.user.id);
      if (
        !user ||
        (user.permissions! < 2 && !server.owners.includes(user.username))
      ) {
        res.status(403);
        res.send("user not authorized for this information");
        return;
      }
      server.status = (await serverStatusPeriphery(server))
        ? "OK"
        : "Could Not Be Reached";
      res.send(server);
    }
  );

  app.get(
    "/api/server/:id/action-state",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const state = app.serverActionStates.getJSON(id);
      res.send(state);
    }
  );

  app.get(
    "/api/server/:id/stats",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const server = await app.servers.findById(id);
      if (!server) {
        res.status(400);
        res.send("server not found");
        return;
      }
      const user = (await app.users.findById(req.user.id))!;
      if (user.permissions! < 1 && !server.owners.includes(user.username)) {
        res.status(403);
        res.send("inadequate permissions");
        return;
      }
      const stats = server.isCore
        ? await getDockerStatsJson()
        : await getPeripheryDockerStats(server);
      res.send(stats);
    }
  );

  app.get(
    "/api/server/:id/sys-stats",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const server = await app.servers.findById(id);
      if (!server) {
        res.status(400);
        res.send("server not found");
        return;
      }
      const sender = (await app.users.findById(req.user.id))!;
      if (sender.permissions! < 1 && !server.owners.includes(sender.username)) {
        res.status(403);
        res.send("inadequate permissions");
        return;
      }
      const stats = server.isCore
        ? await getSystemStats()
        : await getPeripherySystemStats(server);
      res.send(stats);
    }
  );

  app.get(
    "/api/server/:id/stats-history",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const { offset } = req.query as { offset: number };
      const server = await app.servers.findById(id);
      if (!server) {
        res.status(400);
        res.send("server not found");
        return;
      }
      const sender = (await app.users.findById(req.user.id))!;
      if (sender.permissions! < 1 && !server.owners.includes(sender.username)) {
        res.status(403);
        res.send("inadequate permissions");
        return;
      }
      const stats = await app.stats.getMostRecent(100, {}, offset);
      res.send(stats);
    }
  );

  app.post(
    "/api/server/:id/:owner",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      // adds an owner to a build
      const { id, owner } = req.params as { id: string; owner: string };
      const sender = (await app.users.findById(req.user.id))!;
      if (sender.permissions! < 1) {
        res.status(403);
        res.send("inadequate permissions");
        return;
      }
      const user = await app.users.findOne({ username: owner });
      if (!user || user.permissions! < 1) {
        res.status(400);
        res.send("invalid user");
        return;
      }
      const server = await app.servers.findById(id);
      if (!server) {
        res.status(400);
        res.send("server not found");
        return;
      }
      if (sender.permissions! < 2 && !server.owners.includes(sender.username)) {
        res.status(403);
        res.send("inadequate permissions");
        return;
      }
      await app.servers.updateById(id, { $push: { owners: owner } });
      app.broadcast(
        SERVER_OWNER_UPDATE,
        { serverID: id },
        app.serverUserFilter(id)
      );
      res.send("owner added");
    }
  );

  app.delete(
    "/api/server/:id/:owner",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      // removes owner from deployment
      const { id, owner } = req.params as { id: string; owner: string };
      const sender = (await app.users.findById(req.user.id))!;
      if (sender.permissions! < 2) {
        res.status(403);
        res.send("inadequate permissions");
        return;
      }
      const server = await app.servers.findById(id);
      if (!server) {
        res.status(400);
        res.send("server not found");
        return;
      }
      await app.servers.updateById(id, { $pull: { owners: owner } });
      app.broadcast(
        SERVER_OWNER_UPDATE,
        { serverID: id },
        app.serverUserFilter(id)
      );
      res.send("owner removed");
    }
  );

  done();
});

export default servers;
