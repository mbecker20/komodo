import { Deployment } from "@monitor/types";
import { clone, execute, mergeCommandLogError, pull } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { remove } from "fs-extra";
import { join } from "path";
import { CONTAINER_REPO_ROOT, SECRETS } from "../config";

const git = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.post("/repo/clone", { onRequest: [app.auth] }, async (req, res) => {
    const { deployment } = req.body as { deployment: Deployment };
    const log = await clone(
      deployment.repo!,
      CONTAINER_REPO_ROOT + deployment.containerName,
      deployment.subfolder,
      deployment.branch,
      deployment.githubAccount &&
        SECRETS.GITHUB_ACCOUNTS[deployment.githubAccount]
    );
    res.send(log);
  });

  app.post("/repo/pull", { onRequest: [app.auth] }, async (req, res) => {
    const body = req.body as { deployment: Deployment };
    const deployment = body.deployment;
    const pullCle = await pull(
      join(CONTAINER_REPO_ROOT, deployment.containerName!),
      deployment.branch
    );
    const onPullCle =
      deployment.onPull &&
      (await execute(
        `cd ${join(
          CONTAINER_REPO_ROOT,
          deployment.containerName!,
          deployment.onPull.path || ""
        )} && ${deployment.onPull.command}`
      ));
    res.send(
      mergeCommandLogError(
        { name: "pull", cle: pullCle },
        { name: "post", cle: onPullCle }
      )
    );
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
