import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { SECRETS } from "../config";

const dockerAccounts = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/api/docker-accounts", { onRequest: [app.auth] }, async (req, res) => {
		const user = await app.users.findById(req.user.id);
		if (!user || user.permissions! < 1) {
			res.status(403);
			res.send("invalid user");
			return;
		}
		res.send(Object.keys(SECRETS.DOCKER_ACCOUNTS));
	});
	done();
});

export default dockerAccounts;