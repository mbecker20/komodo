import { FastifyReply, FastifyRequest } from "fastify";
import { FastifyInstance } from "fastify";
import fastifyJwt from "fastify-jwt";
import fp from "fastify-plugin";
import { SECRETS } from "../config";

declare module "fastify" {
  interface FastifyInstance {
    auth: any;
  }
}

declare module "fastify-jwt" {
  interface FastifyJWT {
    payload: {
      id: string;
    };
    user: {
      id: string;
    };
  }
}

const jwt = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.register(fastifyJwt, {
    secret: SECRETS.JWT.SECRET,
  });

	app.decorate("auth", async (req: FastifyRequest, res: FastifyReply) => {
    try {
      await req.jwtVerify();
    } catch (err) {
      res.status(403);
      res.send("Authorization header malformed or contains invalid JWT");
    }
  });

	done();
});

export default jwt;