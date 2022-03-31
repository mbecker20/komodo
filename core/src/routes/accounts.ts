import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { SECRETS } from "../config";

const accounts = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get(
    "/api/accounts/docker",
    { onRequest: [app.auth] },
    async (req, res) => {
      const user = await app.users.findById(req.user.id);
      if (!user || user.permissions! < 1) {
        res.status(403);
        res.send("invalid user");
        return;
      }
      res.send(Object.keys(SECRETS.DOCKER_ACCOUNTS));
    }
  );
  app.get(
    "/api/accounts/github",
    { onRequest: [app.auth] },
    async (req, res) => {
      const user = await app.users.findById(req.user.id);
      if (!user || user.permissions! < 1) {
        res.status(403);
        res.send("invalid user");
        return;
      }
      res.send(Object.keys(SECRETS.GITHUB_ACCOUNTS));
    }
  );
  done();
});

export default accounts;
