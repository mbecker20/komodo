import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { getNetworks } from "../util/networks";

const networks = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/api/networks/:serverID", { onRequest: [app.auth] }, async (req, res) => {
		const { serverID } = req.params as { serverID: string };
		const networks = await getNetworks(app);
		res.send(networks);
	});
	done();
});

export default networks;