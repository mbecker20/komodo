import { Build, User } from "@monitor/types";
import { CLONE_BUILD_REPO, mergeCommandLogError } from "@monitor/util";
import { join } from "path";
import { FastifyInstance } from "fastify";
import { BUILD_REPO_PATH, SECRETS } from "../../config";
import { addBuildUpdate } from "../../util/updates";
import { clone, execute } from "@monitor/util-node";

async function cloneRepo(
  app: FastifyInstance,
  user: User,
  { pullName, branch, repo, subfolder, onClone, githubAccount, _id }: Build
) {
  const cloneCle = await clone(
    repo!,
    join(BUILD_REPO_PATH, pullName!),
    subfolder,
    branch,
    githubAccount && SECRETS.GITHUB_ACCOUNTS[githubAccount]
  );
  const onCloneCle =
    onClone &&
    (await execute(
      `cd ${join(BUILD_REPO_PATH, pullName!, onClone.path || "")} && ${
        onClone.command
      }`
    ));
  const { command, log, isError } = mergeCommandLogError(
    {
      name: "clone",
      cle: cloneCle,
    },
    { name: "post clone", cle: onCloneCle }
  );
  addBuildUpdate(
    app,
    _id!,
    CLONE_BUILD_REPO,
    command,
    log,
    user.username,
    "",
    isError
  );
}

export default cloneRepo;
