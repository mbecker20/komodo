import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { REGISTRY_URL, REPO_PATH, SYSTEM_OPERATOR } from "../config";
import { pull, dockerBuild } from "@monitor/util";
import { addBuildUpdate, addDeploymentUpdate } from "../util/updates";

const AUTO_PULL = "Auto Pull";
const AUTO_BUILD = "Auto Build";

const githubListener = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.post("/githubListener", async (req, res) => {
    const query = req.params as { imageName?: string; containerName?: string };
    if (query.imageName) {
      const build = await app.builds.findOne({ imageName: query.imageName });
      if (build) {
        const { _id, buildPath, dockerfilePath, branch, imageName } = build;
        const {
          command: pullCommand,
          log: pullLog,
          isError: pullIsError,
        } = await pull(REPO_PATH + imageName, branch);
        if (!pullIsError && buildPath && dockerfilePath) {
          const {
            command: buildCommand,
            log: buildLog,
            isError: buildIsError,
          } = await dockerBuild(build, REPO_PATH, REGISTRY_URL);
          await addBuildUpdate(
            app,
            _id!,
            AUTO_BUILD,
            `Pull: ${pullCommand}\n\nBuild: ${buildCommand}`,
            {
              stdout: pullLog.stdout + "\n\n" + buildLog.stdout,
              stderr: pullLog.stderr + "\n\n" + buildLog.stderr,
            },
            SYSTEM_OPERATOR,
            "",
            pullIsError || buildIsError
          );
        } else {
          // no docker build associated
          await addBuildUpdate(
            app,
            _id!,
            AUTO_PULL,
            pullCommand,
            pullLog,
            SYSTEM_OPERATOR,
            "",
            pullIsError
          );
        }
      }
    } else if (query.containerName) {
      // needs to be updated to support remote repos attached to deployments (for static frontend)
      const deployment = await app.deployments.findOne({
        containerName: query.containerName,
      });
      if (deployment) {
        const { _id, containerName, branch } = deployment;
        const { command, log, isError } = await pull(
          REPO_PATH + containerName,
          branch
        );
        await addDeploymentUpdate(
          app,
          _id!,
          AUTO_PULL,
          command,
          log,
          SYSTEM_OPERATOR,
          "",
          isError
        );
      }
    }
  });

  done();
});

export default githubListener;
