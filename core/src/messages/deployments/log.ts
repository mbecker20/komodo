import { getContainerLog } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { getPeripheryContainerLog } from "../../util/periphery/container";

async function containerLog(app: FastifyInstance, { deploymentID, tail }: { deploymentID: string; tail?: number }) {
	const deployment = await app.deployments.findById(deploymentID);
	if (!deployment) return "could not find deployment";
	const server = await app.servers.findById(deployment.serverID!);
	if (!server) return "could not find server";
	return server.isCore
    ? await getContainerLog(deployment.containerName!, tail)
    : await getPeripheryContainerLog(server, deployment.containerName!);
}

export default containerLog;