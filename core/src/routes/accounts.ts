import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { SECRETS } from "../config";

const accounts = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get(
    "/api/accounts/docker",
    { onRequest: [app.auth] },
    async (req, res) => {
      const user = await app.users.findById(req.user.id);
      if (!user || !user.enabled || user.permissions! < 1) {
        res.status(403);
        res.send("permission denied");
        return;
      }
      const accounts = await app.accounts.find(
        user.permissions! < 2
          ? { type: "docker", users: user.username }
          : { type: "docker" }
      );
      res.send(accounts.map((act) => act.username));
    }
  );
  app.get(
    "/api/accounts/github",
    { onRequest: [app.auth] },
    async (req, res) => {
      const user = await app.users.findById(req.user.id);
      if (!user || !user.enabled || user.permissions! < 1) {
        res.status(403);
        res.send("invalid user");
        return;
      }
      const accounts = await app.accounts.find(
        user.permissions! < 2
          ? { type: "github", users: user.username }
          : { type: "github" }
      );
      res.send(accounts.map((act) => act.username));
    }
  );
  done();
});

export default accounts;
