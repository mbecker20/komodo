import { filterOutUndefined, USER_UPDATE } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const users = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get("/api/users", { onRequest: [app.auth] }, async (req, res) => {
    const user = await app.users.findById(req.user.id);
    if (!user || !user.enabled || user.permissions! < 1) {
      res.status(403);
      res.send("not authorized");
    }
    const { username, onlyUsers } = req.query as {
      username: string;
      onlyUsers: boolean;
    };
    const users = await app.users.find(
      filterOutUndefined({
        permissions: onlyUsers ? { $lt: 2, $gt: 0 } : { $lt: 2 },
        username: username
          ? {
              $regex: `.*${username}.*`,
            }
          : undefined,
      }),
      "username permissions enabled"
    );
    res.send(users);
  });

  app.delete("/api/user/:id", { onRequest: [app.auth] }, async (req, res) => {
    const admin = await app.users.findById(req.user.id);
    if (!admin || !admin.enabled || admin.permissions! < 2) {
      res.status(403);
      res.send("not authorized");
    }
    const { id } = req.params as { id: string };
    const toDelete = await app.users.findById(id, "username permissions");
    if (!toDelete) {
      res.status(400);
      res.send("user not found");
      return;
    }
    if (toDelete.permissions! > 1) {
      res.status(403);
      res.send("cannot delete admin");
      return;
    }
    await app.users.findByIdAndDelete(id);
    app.deployments.updateMany({}, { $pull: { owners: toDelete.username } });
    app.builds.updateMany({}, { $pull: { owners: toDelete.username } });
    app.broadcast(USER_UPDATE, {});
    res.send(`deleted user ${id} (${toDelete.username})`);
  });

  app.post("/api/user/update", { onRequest: [app.auth] }, async (req, res) => {
    const admin = await app.users.findById(req.user.id);
    if (!admin || !admin.enabled || admin.permissions! < 2) {
      res.status(403);
      res.send("not authorized");
    }
    const { userID, enabled, permissions } = req.body as {
      userID: string;
      enabled?: boolean;
      permissions?: number;
    };
    const user = await app.users.findById(userID);
    if (!user) {
      res.status(400);
      res.send("could not find user");
      return;
    }
    if (user.permissions! > 1) {
      res.status(400);
      res.send("admins can't update other admins");
      return;
    }
    if (permissions && permissions > 2) {
      res.status(400);
      res.send("permissions too high");
      return;
    }
    const update = filterOutUndefined({ enabled, permissions });
    await app.users.updateById(userID, filterOutUndefined(update));
    app.broadcast(USER_UPDATE, {});
    res.send(
      `set user at ${userID} to ${Object.keys(update)
        .map((field) => `${field}: ${update[field]}`)
        .reduce((prev, curr) => prev + ", " + curr)}`
    );
  });

  done();
});

export default users;
