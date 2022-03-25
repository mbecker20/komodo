import { getNetworks } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const networks = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/networks", { onRequest: [app.auth] }, async (_, res) => {
		const networks = await getNetworks(app.dockerode);
		res.send(networks);
	});
	done();
});

export default networks;