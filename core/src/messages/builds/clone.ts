import { Build, User } from "@monitor/types";
import {
  clone,
  CLONE_BUILD_REPO,
  execute,
  mergeCommandLogError,
} from "@monitor/util";
import { join } from "path";
import { FastifyInstance } from "fastify";
import { BUILD_REPO_PATH } from "../../config";
import { addBuildUpdate } from "../../util/updates";

async function cloneRepo(
  app: FastifyInstance,
  user: User,
  { pullName, branch, repo, subfolder, onClone, accessToken, _id }: Build
) {
  const cloneCle = await clone(
    repo!,
    BUILD_REPO_PATH + pullName!,
    subfolder,
    branch,
    accessToken
  );
  const onCloneCle =
    onClone &&
    (await execute(
      `cd ${join(BUILD_REPO_PATH, pullName!, onClone.path || "")} && ${onClone.command}`
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
    !isError
  );
}

export default cloneRepo;
