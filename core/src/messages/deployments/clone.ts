import { Deployment, User } from "@monitor/types";
import { clone } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { CLONE_REPO } from ".";
import { DEPLOYMENT_REPO_PATH } from "../../config";
import { clonePeriphery } from "../../util/periphery/git";
import { addDeploymentUpdate } from "../../util/updates";

async function cloneRepo(
  app: FastifyInstance,
  user: User,
  deployment: Deployment
) {
  const { serverID, containerName, branch, repo, accessToken, _id } =
    deployment;
  const server = serverID ? await app.servers.findById(serverID) : undefined;
  const { command, log, isError } = server
    ? await clonePeriphery(server.address, server.passkey, deployment)
    : await clone(
        repo!,
        DEPLOYMENT_REPO_PATH + containerName!,
        branch,
        accessToken
      );
  addDeploymentUpdate(
    app,
    _id!,
    CLONE_REPO,
    command,
    log,
    user.username,
    "",
    isError
  );
}

export default cloneRepo;
