import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { refreshSecrets } from "../config";

const secrets = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/api/refresh-secrets", { onRequest: [app.auth] }, async (req, res) => {
		const user = await app.users.findById(req.user.id);
		if (user && user.permissions! >= 2) {
			refreshSecrets();
			res.send("refreshed secrets");
		} else {
			res.status(403);
			res.send("invalid user");
		}
	});
	done();
});

export default secrets;