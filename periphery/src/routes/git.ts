import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const git = fp((app: FastifyInstance, _: {}, done: () => void) => {
	
	
	done();
});

export default git;