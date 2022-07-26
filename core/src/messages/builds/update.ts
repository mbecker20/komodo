import { Build, User } from "@monitor/types";
import { buildChangelog, prettyStringify, UPDATE_BUILD } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { move, pathExists, remove } from "fs-extra";
import { WebSocket } from "ws";
import { PERMISSIONS_DENY_LOG, BUILD_REPO_PATH } from "../../config";
import { sendAlert, toDashedName } from "../../util/helpers";
import { addBuildUpdate } from "../../util/updates";
import cloneRepo from "./clone";

async function updateBuild(
  app: FastifyInstance,
  client: WebSocket,
  user: User,
  { build, note }: { build: Build; note?: string }
) {
  if (app.buildActionStates.busy(build._id!)) {
    sendAlert(client, "bad", "build busy, try again in a bit");
    return;
  }
  const preBuild = await app.builds.findById(build._id!).catch(() => {});
  if (!preBuild) return; // may want to add some update here
  if (user.permissions! < 2 && !build.owners.includes(user.username)) {
    addBuildUpdate(
      app,
      build._id!,
      UPDATE_BUILD,
      "Update Build (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  app.broadcast(
    UPDATE_BUILD,
    { buildID: build._id, complete: false },
    app.buildUserFilter(build._id!)
  );
  app.buildActionStates.set(build._id!, "updating", true);
  try {
    build.pullName = toDashedName(build.name);
    if (user.permissions! < 2) {
      // disallow non-admins from updating the onClone / onPull commands
      build.onClone = undefined;
      build.cliBuild = undefined;
    }
    if (build.repo !== preBuild.repo || build.branch !== preBuild.branch) {
      // reclone repo if repo is changed
      await remove(BUILD_REPO_PATH + preBuild.pullName).catch();
      if (build.repo) await cloneRepo(app, user, build);
    } else if (build.pullName !== preBuild.pullName) {
      if (await pathExists(BUILD_REPO_PATH + preBuild.pullName)) {
        await move(
          BUILD_REPO_PATH + preBuild.pullName,
          BUILD_REPO_PATH + build.pullName
        );
      }
      // maybe do something more with deployments
    }
    (build.owners as any) = undefined;
    await app.builds.updateOne({ _id: build._id }, build);
    addBuildUpdate(
      app,
      build._id!,
      UPDATE_BUILD,
      "Update Build",
      { stdout: buildChangelog(preBuild, build) },
      user.username,
      note
    );
    app.buildActionStates.set(build._id!, "updating", false);
    app.broadcast(
      UPDATE_BUILD,
      { buildID: build._id, complete: true },
      app.buildUserFilter(build._id!)
    );
    return build;
  } catch (error) {
    addBuildUpdate(
      app,
      build._id!,
      UPDATE_BUILD,
      "Update Build (ERROR)",
      {
        stderr: prettyStringify(error),
      },
      user.username,
      note,
      true
    );
    app.buildActionStates.set(build._id!, "updating", false);
    app.broadcast(
      UPDATE_BUILD,
      { buildID: build._id, complete: true },
      app.buildUserFilter(build._id!)
    );
  }
}

export default updateBuild;
