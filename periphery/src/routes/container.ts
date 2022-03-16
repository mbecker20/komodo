import { allContainerStatus, deleteContainer, getContainerLog, getContainerStatus, startContainer, stopContainer } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const container = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get("/containers", { onRequest: [app.auth] }, async (_, res) => {
    const containers = await allContainerStatus(app.dockerode);
    res.send(containers);
  });

  app.get("/container/:name", { onRequest: [app.auth] }, async (req, res) => {
    const params = req.params as any;
    const container = await getContainerStatus(app.dockerode, params.name);
    return res.send(container);
  });

  app.get("/container/log/:name", { onRequest: [app.auth] }, async (req, res) => {
    const params = req.params as any;
    const log = await getContainerLog(params.name, params.tail);
    res.send(log);
  });

  app.get("/container/start/:name", { onRequest: [app.auth] }, async (req, res) => {
    const params = req.params as any;
    const log = await startContainer(params.name);
    res.send(log);
  });

  app.get("/container/stop/:name", { onRequest: [app.auth] }, async (req, res) => {
    const params = req.params as any;
    const log = await stopContainer(params.name);
    res.send(log);
  });

  app.get("/container/delete/:name", { onRequest: [app.auth] }, async (req, res) => {
    const params = req.params as any;
    const log = await deleteContainer(params.name);
    res.send(log);
  });

  done();
});

export default container;
