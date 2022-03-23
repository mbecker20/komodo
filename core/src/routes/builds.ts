import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const builds = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/api/builds", { onRequest: [app.auth] }, async (req, res) => {
		const builds = await app.builds.findCollection({}, "name");
		res.send(builds);
	});

	app.get("/api/build/:id", { onRequest: [app.auth] }, async (req, res) => {
		const { id } = req.params as { id: string };
		const build = await app.builds.findById(id);
    res.send(build);
	});
	done();
});

export default builds;