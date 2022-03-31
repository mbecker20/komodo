import { Build, User } from "@monitor/types";
import { buildChangelog, UPDATE_BUILD } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { move, pathExists, remove } from "fs-extra";
import { PERMISSIONS_DENY_LOG, BUILD_REPO_PATH } from "../../config";
import { toDashedName } from "../../util/helpers";
import { addBuildUpdate } from "../../util/updates";
import cloneRepo from "./clone";

async function updateBuild(
  app: FastifyInstance,
  user: User,
  { build, note }: { build: Build; note?: string }
) {
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
  try {
    build.pullName = toDashedName(build.name);
    if (build.repo !== preBuild.repo || build.branch !== preBuild.branch) {
      // reclone repo if repo is changed
      await remove(BUILD_REPO_PATH + preBuild.pullName).catch();
      if (build.repo) {
        app.broadcast(UPDATE_BUILD, { buildID: build._id, complete: false });
        app.buildActionStates.set(build._id!, "updating", true);
        await cloneRepo(app, user, build);
        app.buildActionStates.set(build._id!, "updating", false);
        app.broadcast(UPDATE_BUILD, { buildID: build._id, complete: true });
      }
    } else if (build.pullName !== preBuild.pullName) {
      if (await pathExists(BUILD_REPO_PATH + preBuild.pullName)) {
        await move(BUILD_REPO_PATH + preBuild.pullName, BUILD_REPO_PATH + build.pullName);
      }
      // maybe do something more with deployments
    }
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
    return build;
  } catch (error) {
    addBuildUpdate(
      app,
      build._id!,
      UPDATE_BUILD,
      "Update Build (ERROR)",
      {
        stderr: JSON.stringify(error),
      },
      user.username,
      note,
      true
    );
  }
}

export default updateBuild;
