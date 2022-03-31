import { User } from "@monitor/types";
import {
  dockerBuild,
  BUILD,
  execute,
  mergeCommandLogError,
  prettyStringify,
} from "@monitor/util";
import { join } from "path";
import { FastifyInstance } from "fastify";
import {
  PERMISSIONS_DENY_LOG,
  BUILD_REPO_PATH,
  SECRETS,
} from "../../config";
import { BUILDING } from "../../plugins/actionStates";
import { addBuildUpdate } from "../../util/updates";

async function build(
  app: FastifyInstance,
  user: User,
  { buildID, note }: { buildID: string; note?: string }
) {
  const build = await app.builds.findById(buildID);
  if (!build) return;
  if (user.permissions! < 2 && !build.owners.includes(user.username)) {
    addBuildUpdate(
      app,
      buildID,
      BUILD,
      "Build (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  if (!app.buildActionStates.get(buildID, BUILDING)) {
    app.buildActionStates.set(buildID, BUILDING, true);
    app.broadcast(BUILD, { complete: false, buildID });
    const { cliBuild, dockerBuildArgs } = build;
    try {
      const pull = await execute(
        `cd ${join(BUILD_REPO_PATH, build.pullName || "")} && git pull origin ${
          build.branch || "main"
        }`
      );
      const cli =
        cliBuild &&
        (await execute(
          `cd ${join(
            BUILD_REPO_PATH,
            build.pullName!,
            cliBuild.path || ""
          )} && ${cliBuild.command}`
        ));
      const docker =
        dockerBuildArgs &&
        (await dockerBuild(
          build.pullName!,
          dockerBuildArgs,
          BUILD_REPO_PATH,
          build.dockerAccount,
          build.dockerAccount && SECRETS.DOCKER_ACCOUNTS[build.dockerAccount]
        ));
      const { command, log, isError } = mergeCommandLogError(
        { name: "pull", cle: pull },
        { name: "cli", cle: cli },
        { name: "docker", cle: docker }
      );
      addBuildUpdate(
        app,
        buildID,
        BUILD,
        command,
        log,
        user.username,
        note,
        isError
      );
    } catch (error) {
      addBuildUpdate(
        app,
        buildID,
        BUILD,
        "Build (ERROR)",
        { stderr: prettyStringify(error) },
        user.username,
        note,
        true
      );
    }
    app.broadcast(BUILD, { complete: true, buildID });
    app.buildActionStates.set(buildID, BUILDING, false);
  }
}

export default build;
