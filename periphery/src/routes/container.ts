import { allContainerStatus, deleteContainer, getContainerLog, getContainerStatus, startContainer, stopContainer } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const container = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get("/containers", { onRequest: [app.auth] }, async (_, res) => {
    const containers = await allContainerStatus(app.dockerode);
    res.send(containers);
  });

  app.get("/container/:name", { onRequest: [app.auth] }, async (req, res) => {
    const { name } = req.params as { name: string };
    const container = await getContainerStatus(app.dockerode, name);
    return res.send(container);
  });

  app.get("/container/log/:name", { onRequest: [app.auth] }, async (req, res) => {
    const { name } = req.params as { name: string };
    const { tail } = req.query as { tail: number };
    const log = await getContainerLog(name, tail);
    res.send(log);
  });

  app.get("/container/start/:name", { onRequest: [app.auth] }, async (req, res) => {
    const { name } = req.params as { name: string };
    const log = await startContainer(name);
    res.send(log);
  });

  app.get("/container/stop/:name", { onRequest: [app.auth] }, async (req, res) => {
    const { name } = req.params as { name: string };
    const log = await stopContainer(name);
    res.send(log);
  });

  app.get("/container/delete/:name", { onRequest: [app.auth] }, async (req, res) => {
    const { name } = req.params as { name: string };
    const log = await deleteContainer(name);
    res.send(log);
  });

  done();
});

export default container;
