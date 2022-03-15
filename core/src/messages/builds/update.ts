import { Build, User } from "@monitor/types";
import { buildChangelog } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { move, pathExists, remove } from "fs-extra";
import { UPDATE_BUILD } from ".";
import { PERMISSIONS_DENY_LOG, REPO_PATH } from "../../config";
import { toDashedName } from "../../util/helpers";
import { addBuildUpdate, addSystemUpdate } from "../../util/updates";
import cloneRepo from "./clone";

async function updateBuild(
  app: FastifyInstance,
  user: User,
  { build, note }: { build: Build; note?: string }
) {
  const preBuild = await app.builds.findById(build._id!).catch(() => {});
  if (!preBuild) return; // may want to add some update here
  if (user.permissions! >= 2 || user.username === preBuild.owner) {
    try {
      build.imageName = toDashedName(build.name);
      if (build.repo !== preBuild.repo || build.branch !== preBuild.branch) {
        // reclone repo if repo is changed
        await remove(REPO_PATH + preBuild.imageName).catch();
        if (build.repo) {
          await cloneRepo(app, user, build);
        }
      } else if (build.imageName !== preBuild.imageName) {
        if (await pathExists(REPO_PATH + preBuild.imageName)) {
          await move(
            REPO_PATH + preBuild.imageName,
            REPO_PATH + build.imageName
          );
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
  } else {
    addSystemUpdate(
      app,
      UPDATE_BUILD,
      "Update Build (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
  }
}

export default updateBuild;
