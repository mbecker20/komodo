import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { REPO_PATH, SYSTEM_OPERATOR } from "../config";
import { execute } from "@monitor/util";
import { createDockerBuild } from "../util/docker/build";
import { addBuildUpdate, addDeploymentUpdate } from "../util/updates";

const AUTO_PULL = "Auto Pull";
const AUTO_BUILD = "Auto Build";

const githubListener = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.post("/githubListener", async (req, res) => {
    const query = req.params as { pullName?: string; containerName?: string };
    if (query.pullName) {
      const build = await app.builds.findOne({ pullName: query.pullName });
      if (build) {
        const { _id, buildPath, dockerfilePath, branch, pullName } = build;
        const pullCommand = `cd ${REPO_PATH}${pullName} && git pull origin ${
          branch ? branch : "master"
        }`;
        const { log: pullLog, success: pullSuccess } = await execute(
          pullCommand
        );
        if (buildPath && dockerfilePath) {
          const buildCommand = createDockerBuild(build);
          const { log: buildLog, success: buildSuccess } = await execute(
            buildCommand
          );
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
            !(pullSuccess && buildSuccess)
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
            !pullSuccess
          );
        }
      }
    } else if (query.containerName) {
      const deployment = await app.deployments.findOne({
        containerName: query.containerName,
      });
      if (deployment) {
        const { _id, containerName, branch } = deployment;
        const command = `cd ${REPO_PATH}${containerName} && git pull origin ${
          branch ? branch : "master"
        }`;
        const { log, success } = await execute(command);
        await addDeploymentUpdate(
          app,
          _id!,
          AUTO_PULL,
          command,
          log,
          SYSTEM_OPERATOR,
          "",
          !success
        );
      }
    }
  });

  done();
});

export default githubListener;
