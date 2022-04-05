import { Deployment } from "@monitor/types";
import { deleteContainer, dockerRun } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { join } from "path";
import { SECRETS, SYSROOT, SYS_REPO_ROOT } from "../config";

const deploy = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.post("/deploy", { onRequest: [app.auth] }, async (req, res) => {
    const { deployment } = req.body as { deployment: Deployment };
    // send the image name in the query if deployment is of a monitor build
    const { image } = req.query as { image?: string };
    const repoMount =
      deployment.repo && deployment.containerMount
        ? {
            repoFolder: join(
              SYS_REPO_ROOT,
              deployment.containerName!,
              deployment.repoMount || ""
            ),
            containerMount: deployment.containerMount!,
          }
        : undefined;
    await deleteContainer(deployment.containerName!);
    const log = await dockerRun(
      {
        ...deployment,
        image: image
          ? join(deployment.dockerAccount || "", image)
          : deployment.image,
      },
      SYSROOT,
      repoMount,
      deployment.dockerAccount,
      deployment.dockerAccount &&
        SECRETS.DOCKER_ACCOUNTS[deployment.dockerAccount]
    );
    res.send(log);
  });

  done();
});

export default deploy;
