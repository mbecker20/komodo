import { BUILD_OWNER_UPDATE } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const builds = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get(
    "/api/builds",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const user = (await app.users.findById(req.user.id))!;
      const builds = await app.builds.findCollection(
        user.permissions! > 1 ? {} : { owners: user.username },
        "name owners"
      );
      res.send(builds);
    }
  );

  app.get(
    "/api/build/:id",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const user = (await app.users.findById(req.user.id))!;
      const build = await app.builds.findById(id);
      if (!build) {
        res.status(400);
        res.send("build not found");
        return;
      }
      if (user.permissions! < 2 && !build.owners.includes(user.username)) {
        res.status(403);
        res.send("access denied");
        return;
      }
      res.send(build);
    }
  );

  app.get(
    "/api/build/:id/action-state",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const state = app.buildActionStates.getJSON(id);
      res.send(state);
    }
  );

  app.post(
    "/api/build/:id/:owner",
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
      const build = await app.builds.findById(id);
      if (!build) {
        res.status(400);
        res.send("build not found");
        return;
      }
      if (sender.permissions! < 2 && !build.owners.includes(sender.username)) {
        res.status(403);
        res.send("inadequate permissions");
        return;
      }
      await app.builds.updateById(id, { $push: { owners: owner } });
      app.broadcast(BUILD_OWNER_UPDATE, { buildID: id });
      res.send("owner added");
    }
  );

  app.delete(
    "/api/build/:id/:owner",
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
      const build = await app.builds.findById(id);
      if (!build) {
        res.status(400);
        res.send("build not found");
        return;
      }
      await app.builds.updateById(id, { $pull: { owners: owner } });
      app.broadcast(BUILD_OWNER_UPDATE, { buildID: id });
      res.send("owner removed");
    }
  );

  done();
});

export default builds;
