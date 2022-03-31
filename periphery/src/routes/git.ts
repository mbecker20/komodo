import { Deployment } from "@monitor/types";
import { clone, pull } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { remove } from "fs-extra";
import { CONTAINER_REPO_ROOT, SECRETS } from "../config";

const git = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.post("/repo/clone", { onRequest: [app.auth] }, async (req, res) => {
    const body = req.body as { deployment: Deployment };
    const deployment = body.deployment;
    const log = await clone(
      deployment.repo!,
      CONTAINER_REPO_ROOT + deployment.containerName,
      deployment.subfolder,
      deployment.branch,
      deployment.githubAccount && SECRETS.GITHUB_ACCOUNTS[deployment.githubAccount]
    );
		res.send(log);
  });

	app.post("/repo/pull", { onRequest: [app.auth] }, async (req, res) => {
		const body = req.body as { deployment: Deployment };
    const deployment = body.deployment;
		const log = await pull(CONTAINER_REPO_ROOT + deployment.containerName, deployment.branch);
		res.send(log);
	});

	app.post("/repo/delete", { onRequest: [app.auth] }, async (req, res) => {
		const body = req.body as { deployment: Deployment };
    const deployment = body.deployment;
		await remove(CONTAINER_REPO_ROOT + deployment.containerName).catch();
		res.send();
	});

  done();
});

export default git;
