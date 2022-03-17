import { Deployment } from "@monitor/types";
import { deleteContainer, dockerRun } from "@monitor/util";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { REGISTRY_URL, SYSROOT, SYS_REPO_ROOT } from "../config";

const deploy = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.post("/deploy", { onRequest: [app.auth] }, async (req, res) => {
    const { deployment } = req.body as { deployment: Deployment };
    // send the image name in the query if deployment is of a monitor build
    const { image } = req.query as { image?: string };
    const repoMount =
      deployment.repo && deployment.containerMount
        ? {
            repoFolder: SYS_REPO_ROOT,
            containerMount: deployment.containerMount!,
          }
        : undefined;
    await deleteContainer(deployment.containerName!);
    const log = await dockerRun(
      {
        ...deployment,
        image: image ? REGISTRY_URL + image : deployment.image,
        latest: image ? true : deployment.latest,
      },
      SYSROOT,
      repoMount
    );
    res.send(log);
  });

  done();
});

export default deploy;
