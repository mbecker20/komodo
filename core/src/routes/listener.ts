import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { join } from "path";
import {
  BUILD_REPO_PATH,
  DEPLOYMENT_REPO_PATH,
  SECRETS,
  SYSTEM_OPERATOR,
} from "../config";
import {
  pull,
  dockerBuild,
  mergeCommandLogError,
  execute,
} from "@monitor/util";
import { addBuildUpdate, addDeploymentUpdate } from "../util/updates";
import { pullPeriphery } from "../util/periphery/git";

const AUTO_PULL = "AUTO_PULL";
const AUTO_BUILD = "AUTO_BUILD";

const listener = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.post("/api/listener/build/:buildID", async (req, res) => {
    const { buildID } = req.params as { buildID: string };
    const build = await app.builds.findById(buildID);
    if (!build) {
      res.status(400);
      res.send();
      return;
    }
    const { dockerBuildArgs, branch, pullName, dockerAccount, cliBuild } =
      build;
    const pullCle = await pull(join(BUILD_REPO_PATH, pullName!), branch);
    const cliBuildCle =
      cliBuild &&
      (await execute(
        `cd ${join(BUILD_REPO_PATH, pullName!, cliBuild.path || "")} && ${
          cliBuild.command
        }`
      ));
    const dockerBuildCle =
      dockerBuildArgs &&
      (await dockerBuild(
        pullName!,
        dockerBuildArgs,
        BUILD_REPO_PATH,
        dockerAccount,
        dockerAccount && SECRETS.DOCKER_ACCOUNTS[dockerAccount]
      ));
    const { command, log, isError } = mergeCommandLogError(
      { name: "pull", cle: pullCle },
      { name: "cli build", cle: cliBuildCle },
      { name: "docker build", cle: dockerBuildCle }
    );
    await addBuildUpdate(
      app,
      buildID,
      AUTO_BUILD,
      command,
      log,
      SYSTEM_OPERATOR,
      "",
      isError
    );
    res.send();
  });

  app.post("/api/listener/deployment/:deploymentID", async (req, res) => {
    const { deploymentID } = req.params as { deploymentID: string };
    const deployment = await app.deployments.findById(deploymentID);
    if (!deployment) {
      res.status(400);
      res.send();
      return;
    }
    const { branch, containerName, onPull, serverID } = deployment;
    const server = await app.servers.findById(serverID!);
    if (!server) {
      res.status(400);
      res.send();
      return;
    }
    if (server.isCore) {
      const pullCle = await pull(join(BUILD_REPO_PATH, containerName!), branch);
      const onPullCle =
        onPull &&
        (await execute(
          `cd ${join(
            DEPLOYMENT_REPO_PATH,
            containerName!,
            onPull.path || ""
          )} && ${onPull.command}`
        ));
      const { command, log, isError } = mergeCommandLogError(
        { name: "pull", cle: pullCle },
        { name: "on pull", cle: onPullCle }
      );
      await addDeploymentUpdate(
        app,
        deploymentID,
        AUTO_PULL,
        command,
        log,
        SYSTEM_OPERATOR,
        "",
        isError
      );
      res.send();
    } else {
      const { command, log, isError } = await pullPeriphery(server, deployment);
      await addDeploymentUpdate(
        app,
        deploymentID,
        AUTO_PULL,
        command,
        log,
        SYSTEM_OPERATOR,
        "",
        isError
      );
      res.send()
    }
  });

  done();
});

export default listener;
