import {
  intoCollection,
} from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { deploymentStatusLocal } from "../util/deploymentStatus";
import { getPeripheryContainers } from "../util/periphery/container";

const deployments = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get("/deployments", { onRequest: [app.auth] }, async (req, res) => {
    // returns the periphery deployments on the given serverID
    // returns the core deployments if no serverID is specified
    const { serverID } = req.query as { serverID?: string };
    const server = serverID ? await app.servers.findById(serverID) : app.core;
    if (!server) {
      res.status(400);
      res.send();
      return;
    }
    const deployments = await app.deployments.find(
      { serverID: server._id },
      "name containerName serverID"
    );
    const status = server.isCore ? await deploymentStatusLocal(app, deployments) : await getPeripheryContainers(server);
    res.send(
      intoCollection(
        deployments.map((dep) => ({
          ...dep,
          status: status[dep.containerName!],
        }))
      )
    );
  });

  app.get("/deployment/:id", { onRequest: [app.auth] }, async (req, res) => {
    const { id } = req.params as { id: string };
    const deployment = await app.deployments.findById(id);
    res.send(deployment);
  });
  done();
});

export default deployments;
