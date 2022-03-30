import { User } from "@monitor/types";
import {
  dockerBuild,
  BUILD,
  execute,
  mergeCommandLogError,
} from "@monitor/util";
import { FastifyInstance } from "fastify";
import {
  PERMISSIONS_DENY_LOG,
  REGISTRY_URL,
  BUILD_REPO_PATH,
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
  if (app.buildActionStates.get(buildID, BUILDING)) {
    app.buildActionStates.set(buildID, BUILDING, true);
    app.broadcast(BUILD, { complete: false, buildID });
    const { cliBuild, dockerBuildArgs } = build;
    try {
      const cli = cliBuild
        ? await execute(
            `cd ${BUILD_REPO_PATH}${
              cliBuild.path ? (cliBuild.path[0] === "/" ? "" : "/") : ""
            }${cliBuild.path} && ${cliBuild.command}`
          )
        : undefined;
      const docker = dockerBuildArgs
        ? await dockerBuild(
            build.pullName!,
            dockerBuildArgs,
            BUILD_REPO_PATH,
            REGISTRY_URL
          )
        : undefined;
      const { command, log, isError } = mergeCommandLogError(
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
        { stderr: JSON.stringify(error) },
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
