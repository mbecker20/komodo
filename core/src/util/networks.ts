import { FastifyInstance } from "fastify";

export async function getNetworks(app: FastifyInstance) {
	return await app.dockerode.listNetworks();
}