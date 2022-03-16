import { User } from "@monitor/types";
import { dockerBuild } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { BUILD } from ".";
import { PERMISSIONS_DENY_LOG, REGISTRY_URL, BUILD_REPO_PATH } from "../../config";
import { BUILDING } from "../../plugins/actionStates";
import { addBuildUpdate } from "../../util/updates";

async function build(
  app: FastifyInstance,
  user: User,
  { buildID, note }: { buildID: string; note?: string }
) {
  const build = await app.builds.findById(buildID);
  if (!build) return;
  if (user.permissions! < 2 && build.owner !== user.username) {
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
    try {
      const { command, log, isError } = await dockerBuild(
        build,
        BUILD_REPO_PATH,
        REGISTRY_URL
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
