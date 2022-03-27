import {
  getContainerLog,
  getContainerStatus,
  intoCollection,
} from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { deploymentStatusLocal } from "../util/deploymentStatus";
import {
  getPeripheryContainer,
  getPeripheryContainerLog,
  getPeripheryContainers,
} from "../util/periphery/container";

const deployments = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get("/api/deployments", { onRequest: [app.auth] }, async (req, res) => {
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
    const status = server.isCore
      ? await deploymentStatusLocal(app)
      : await getPeripheryContainers(server);
    res.send(
      intoCollection(
        deployments.map((deployment) => ({
          ...deployment,
          status: status[deployment.containerName!] || "not created",
        }))
      )
    );
  });

  app.get(
    "/api/deployment/:id",
    { onRequest: [app.auth] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const deployment = await app.deployments.findById(id);
      if (!deployment) {
        res.status(400);
        res.send("could not find deployment");
        return;
      }
      const onCore = deployment.serverID === app.core._id;
      const server = onCore
        ? app.core
        : await app.servers.findById(deployment.serverID!);
      if (!server) {
        res.status(400);
        res.send("could not find deployment's server");
        return;
      }
      deployment.status = onCore
        ? await getContainerStatus(app.dockerode, deployment.containerName!)
        : await getPeripheryContainer(server, deployment.containerName!);
      res.send(deployment);
    }
  );
  done();

  app.get(
    "/api/deployment/:id/log",
    { onRequest: [app.auth] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const { tail } = req.query as { tail?: number };
      const deployment = await app.deployments.findById(
        id,
        "serverID containerName"
      );
      if (!deployment) {
        res.status(400);
        res.send("could not find deployment");
        return;
      }
      const onCore = deployment.serverID === app.core._id;
      const server = onCore
        ? app.core
        : await app.servers.findById(deployment.serverID!);
      if (!server) {
        res.status(400);
        res.send("could not find deployment's server");
        return;
      }
      const log = onCore
        ? await getContainerLog(deployment.containerName!, tail || 50)
        : await getPeripheryContainerLog(server, deployment.containerName!, tail || 50);
      res.send(log);
    }
  );
});

export default deployments;
