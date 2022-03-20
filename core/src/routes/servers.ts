import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const servers = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/servers", { onRequest: [app.auth] }, async (req, res) => {
		const servers = await app.servers.findCollection({});
		res.send(servers);
	});
	done();
});

export default servers;