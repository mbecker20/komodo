import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import removeServer from "../messages/servers/remove";

const updates = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get(
    "/api/updates",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      // serves the last 10 updates going back an optional offest
      const { offset, buildID, serverID, deploymentID } = req.query as {
        offset?: number;
        buildID?: string;
        serverID?: string;
        deploymentID?: string;
      };
      const user = await app.users.findById(
        req.user.id,
        "username permissions"
      );
      if (!user) {
        res.status(400);
        res.send("user not found");
        return;
      }
      if (user.permissions! < 1) {
        res.status(403);
        res.send("user does not have permission to access this information");
        return;
      }
      if (user.permissions! > 1) {
        const updates = await app.updates.getMostRecent(
          10,
          buildID
            ? { buildID }
            : deploymentID
            ? { deploymentID }
            : serverID
            ? { serverID }
            : {},
          offset
        );
        res.send(updates);
        return;
      }
      if (buildID) {
        const build = await app.builds.findById(buildID, "owners");
        if (!build || !build.owners.includes(user.username)) {
          res.status(403);
          res.send("user does not have permission to access this data");
          return;
        }
        const updates = await app.updates.getMostRecent(
          10,
          { buildID },
          offset
        );
        res.send(updates);
      } else if (deploymentID) {
        const deployment = await app.deployments.findById(
          deploymentID,
          "owners"
        );
        if (!deployment || !deployment.owners.includes(user.username)) {
          res.status(403);
          res.send("user does not have permission to access this data");
          return;
        }
        const updates = await app.updates.getMostRecent(
          10,
          { deploymentID },
          offset
        );
        res.send(updates);
      } else if (serverID) {
        const server = await app.servers.findById(serverID, "owners");
        if (!server || !server.owners.includes(user.username)) {
          res.status(403);
          res.send("user does not have permission to access this data");
          return;
        }
        const updates = await app.updates.getMostRecent(
          10,
          { serverID },
          offset
        );
        res.send(updates);
      } else {
        const deploymentIDs = (
          await app.deployments.find(
            { owners: { $elemMatch: { $eq: user.username } } },
            "_id"
          )
        ).map((dep) => dep._id);
        const buildIDs = (
          await app.builds.find(
            { owners: { $elemMatch: { $eq: user.username } } },
            "_id"
          )
        ).map((build) => build._id);
        const serverIDs = (
          await app.servers.find(
            { owners: { $elemMatch: { $eq: user.username } } },
            "_id"
          )
        ).map((server) => server._id);
        const updates = await app.updates.getMostRecent(
          10,
          {
            $or: [
              { deploymentID: { $in: deploymentIDs } },
              { buildID: { $in: buildIDs } },
              { serverID: { $in: serverIDs } },
            ],
          },
          offset
        );
        res.send(updates);
      }
    }
  );
  done();
});

export default updates;
