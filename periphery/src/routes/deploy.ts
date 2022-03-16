import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const deploy = fp((app: FastifyInstance, _: {}, done: () => void) => {
	
	done();
});

export default deploy;