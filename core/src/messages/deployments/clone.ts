import { Deployment, User } from "@monitor/types";
import { clone, execute, mergeCommandLogError } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { CLONE_DEPLOYMENT_REPO } from "@monitor/util";
import { DEPLOYMENT_REPO_PATH } from "../../config";
import { clonePeriphery } from "../../util/periphery/git";
import { addDeploymentUpdate } from "../../util/updates";

async function cloneRepo(
  app: FastifyInstance,
  user: User,
  deployment: Deployment
) {
  const {
    serverID,
    containerName,
    branch,
    repo,
    subfolder,
    accessToken,
    onPull,
    _id,
  } = deployment;
  const server =
    serverID === app.core._id
      ? app.core
      : await app.servers.findById(serverID!);
  if (!server) {
    addDeploymentUpdate(
      app,
      _id!,
      CLONE_DEPLOYMENT_REPO,
      "clone (FAILED)",
      { stderr: "server not found" },
      user.username,
      "",
      true
    );
    return;
  }
  const cloneCle = server.isCore
    ? await clonePeriphery(server, deployment)
    : await clone(
        repo!,
        DEPLOYMENT_REPO_PATH + containerName!,
        subfolder,
        branch,
        accessToken
      );
  const onPullCle =
    !server && onPull
      ? await execute(
          `cd ${DEPLOYMENT_REPO_PATH + containerName!}${
            onPull.path ? (onPull.path[0] === "/" ? "" : "/") : ""
          }${onPull.path ? onPull.path : ""} && ${onPull.command}`
        )
      : undefined;
  const { command, log, isError } = mergeCommandLogError(
    {
      name: "clone",
      cle: cloneCle,
    },
    { name: "post clone", cle: onPullCle }
  );
  addDeploymentUpdate(
    app,
    _id!,
    CLONE_DEPLOYMENT_REPO,
    command,
    log,
    user.username,
    "",
    isError
  );
}

export default cloneRepo;
