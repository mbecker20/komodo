import { Deployment } from "@monitor/types";
import { deleteContainer, dockerRun } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { SYSROOT, SYS_REPO_ROOT } from "../config";

const deploy = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.post("/deploy", { onRequest: [app.auth] }, async (req, res) => {
		const body = req.body as { deployment: Deployment };
		const deployment = body.deployment;
		const repoMount = deployment.repo && deployment.containerMount
      ? {
          repoFolder: SYS_REPO_ROOT,
          containerMount: deployment.containerMount!,
        }
      : undefined;
		await deleteContainer(deployment.containerName!);
		const log = await dockerRun(deployment, SYSROOT, repoMount);
		res.send(log);
	});

	done();
});

export default deploy;