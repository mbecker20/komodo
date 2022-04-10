import { User } from "@monitor/types";
import { prettyStringify, pull, PULL_BUILD } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";
import { PERMISSIONS_DENY_LOG, BUILD_REPO_PATH } from "../../config";
import { PULLING } from "../../plugins/actionStates";
import { sendAlert } from "../../util/helpers";
import { addBuildUpdate } from "../../util/updates";

async function pullRepo(
  app: FastifyInstance,
  client: WebSocket,
  user: User,
  { buildID, note }: { buildID: string; note?: string }
) {
  if (app.buildActionStates.busy(buildID)) {
    sendAlert(client, "bad", "build busy, try again in a bit");
    return;
  }
  const build = await app.builds.findById(buildID);
  if (!build) return;
  if (user.permissions! < 2 && !build.owners.includes(user.username)) {
    addBuildUpdate(
      app,
      buildID,
      PULL_BUILD,
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
    app.broadcast(PULL_BUILD, { complete: false, buildID });
    try {
      const { pullName, branch } = build;
      const { command, log, isError } = await pull(
        BUILD_REPO_PATH + pullName,
        branch
      );
      addBuildUpdate(
        app,
        buildID,
        PULL_BUILD,
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
        PULL_BUILD,
        "Pull (ERROR)",
        { stderr: prettyStringify(error) },
        user.username,
        note,
        true
      );
    }
    app.broadcast(PULL_BUILD, { complete: true, buildID });
    app.buildActionStates.set(buildID, PULLING, false);
  }
}

export default pullRepo;
