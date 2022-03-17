import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const builds = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/builds", { onRequest: [app.auth] }, async (req, res) => {
		const builds = await app.builds.find({});
		res.send(builds);
	});
	done();
});

export default builds;