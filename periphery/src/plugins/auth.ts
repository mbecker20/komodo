import { FastifyInstance, FastifyReply, FastifyRequest } from "fastify";
import fp from "fastify-plugin";
import { PASSKEY } from "../config";

declare module "fastify" {
	interface FastifyInstance {
		auth: any;
	}
}

const auth = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.decorate("auth", async (req: FastifyRequest, res: FastifyReply) => {
		try {
			await verifyPasskey(req.headers.authorization)
		} catch{
			res.status(403);
      res.send("unauthorized access to monitor periphery");
		}
	})

	done();
});

function verifyPasskey(passkey?: string): Promise<void> {
	return new Promise((res, rej) => {
		if (passkey === PASSKEY) {
			res();
		} else {
			rej();
		}
	})
}

export default auth;