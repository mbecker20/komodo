import { FastifyInstance, FastifyReply, FastifyRequest } from "fastify";
import fp from "fastify-plugin";
import github from "./github";
import jwt from "./jwt";
import local from "./local";

declare module "fastify" {
	interface FastifyInstance {
		userEnabled: any;
	}
}

const auth = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app
		.register(jwt)
		.register(github)
		.register(local);

	app.decorate(
    "userEnabled",
    async (req: FastifyRequest, res: FastifyReply) => {
      const user = await app.users.findById(req.user.id, "enabled");
      if (!user || !user.enabled) {
				res.status(403);
				res.send("user not enabled");
			}
    }
  );

	done();
});

export default auth;