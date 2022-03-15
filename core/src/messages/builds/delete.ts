import { User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { remove } from "fs-extra";
import { DELETE_BUILD } from ".";
import { PERMISSIONS_DENY_LOG, REPO_PATH } from "../../config";
import { addSystemUpdate } from "../../util/updates";

async function deleteBuild(
  app: FastifyInstance,
  user: User,
  { buildID, note }: { buildID: string; note?: string }
) {
  const build = await app.builds.findById(buildID);
  if (!build) return;
  if (user.permissions! < 2 && user.username !== build.owner) {
    addSystemUpdate(
      app,
      DELETE_BUILD,
      "Delete Build (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  try {
    await app.builds.findByIdAndDelete(buildID);
    await app.deployments.updateMany(
      { buildID: build._id },
      { buildID: undefined }
    );
    if (build!.repo) await remove(REPO_PATH + build.imageName);
    app.buildActionStates.delete(buildID);
    addSystemUpdate(app, DELETE_BUILD, "Delete Build", {}, user.username, note);
    return true;
  } catch (error) {
    addSystemUpdate(
      app,
      DELETE_BUILD,
      "Delete Build",
      {
        stderr: JSON.stringify(error),
      },
      user.username,
      note,
      true
    );
  }
}

export default deleteBuild;
