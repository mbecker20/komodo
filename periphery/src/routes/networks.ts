import { createNetwork, deleteNetwork, getNetworks, pruneNetworks } from "@monitor/util-node";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const networks = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/networks", { onRequest: [app.auth] }, async (_, res) => {
		const networks = await getNetworks(app.dockerode);
		res.send(networks);
	});

	app.get("/networks/prune", { onRequest: [app.auth] }, async (_, res) => {
    const log = await pruneNetworks();
    res.send(log);
  });

	app.get("/network/create/:name", { onRequest: [app.auth] }, async (req, res) => {
		const { name } = req.params as { name: string };
		const { driver } = req.query as { driver?: string }
		const log = await createNetwork(name, driver);
		res.send(log);
	});

	app.get(
    "/network/delete/:name",
    { onRequest: [app.auth] },
    async (req, res) => {
      const { name } = req.params as { name: string };
      const log = await deleteNetwork(name);
      res.send(log);
    }
  );

	done();
});

export default networks;