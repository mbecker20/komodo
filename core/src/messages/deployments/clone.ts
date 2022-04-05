import { Deployment, User } from "@monitor/types";
import { clone, execute, mergeCommandLogError } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { join } from "path";
import { CLONE_DEPLOYMENT_REPO } from "@monitor/util";
import { DEPLOYMENT_REPO_PATH, SECRETS } from "../../config";
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
    githubAccount,
    onPull,
    onClone,
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
  // console.log("cloning repo")
  const cloneCle = server.isCore
    ? await clone(
        repo!,
        DEPLOYMENT_REPO_PATH + containerName!,
        subfolder,
        branch,
        githubAccount && SECRETS.GITHUB_ACCOUNTS[githubAccount]
      )
    : await clonePeriphery(server, deployment);
  const onCloneCle =
    server.isCore && onClone
      ? await execute(
          `cd ${join(
            DEPLOYMENT_REPO_PATH,
            containerName!,
            onClone.path || ""
          )} && ${onClone.command}`
        )
      : undefined;
  const onPullCle =
    server.isCore && onPull
      ? await execute(
          `cd ${join(
            DEPLOYMENT_REPO_PATH,
            containerName!,
            onPull.path || ""
          )} && ${onPull.command}`
        )
      : undefined;
  const { command, log, isError } = mergeCommandLogError(
    {
      name: "clone",
      cle: cloneCle,
    },
    {
      name: "on clone",
      cle: onCloneCle,
    },
    { name: "on pull", cle: onPullCle }
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
