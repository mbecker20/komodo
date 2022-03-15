import { User } from "@monitor/types";
import { pull } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { PULL } from ".";
import { PERMISSIONS_DENY_LOG, REPO_PATH } from "../../config";
import { PULLING } from "../../plugins/actionStates";
import { addBuildUpdate } from "../../util/updates";

async function pullRepo(
  app: FastifyInstance,
  user: User,
  { buildID, note }: { buildID: string; note?: string }
) {
  const build = await app.builds.findById(buildID);
  if (!build) return;
  if (user.permissions! < 2 && user.username !== build.owner) {
    addBuildUpdate(
      app,
      buildID,
      PULL,
      "Pull (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  if (!app.buildActionStates.get(buildID, PULLING)) {
    app.buildActionStates.set(buildID, PULLING, true);
    app.broadcast(PULL, { complete: false, buildID });
    try {
      const { imageName, branch } = build;
      const { command, log, isError } = await pull(
        REPO_PATH + imageName,
        branch
      );
      addBuildUpdate(
        app,
        buildID,
        PULL,
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
        PULL,
        "Pull (ERROR)",
        { stderr: JSON.stringify(error) },
        user.username,
        note,
        true
      );
    }
    app.broadcast(PULL, { complete: true, buildID });
    app.buildActionStates.set(buildID, PULLING, false);
  }
}

export default pullRepo;
