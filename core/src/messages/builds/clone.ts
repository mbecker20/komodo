import { Build, User } from "@monitor/types";
import { clone, CLONE_BUILD_REPO } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { BUILD_REPO_PATH } from "../../config";
import { addBuildUpdate } from "../../util/updates";

async function cloneRepo(
  app: FastifyInstance,
  user: User,
  { imageName, branch, repo, accessToken, _id }: Build
) {
  const { command, log, isError } = await clone(
    repo!,
    BUILD_REPO_PATH + imageName!,
    branch,
    accessToken
  );
  addBuildUpdate(
    app,
    _id!,
    CLONE_BUILD_REPO,
    command,
    log,
    user.username,
    "",
    !isError
  );
}

export default cloneRepo;
