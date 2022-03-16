import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import github from "./github";
import jwt from "./jwt";
import local from "./local";

const auth = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app
		.register(jwt)
		.register(github)
		.register(local);

	done();
});

export default auth;