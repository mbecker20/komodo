import { Build, User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { CREATE_BUILD } from "@monitor/util";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { toDashedName } from "../../util/helpers";
import { addBuildUpdate, addSystemUpdate } from "../../util/updates";
import cloneRepo from "./clone";

async function createBuild(
  app: FastifyInstance,
  user: User,
  message: { build: Build; note?: string }
) {
  if (user.permissions! < 1) {
    addSystemUpdate(
      app,
      CREATE_BUILD,
      "Create Build (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      message.note,
      true
    );
    return;
  }
  try {
    const build = await app.builds.create({
      ...message.build,
      imageName: toDashedName(message.build.name),
      owner: user.username,
    });
    app.buildActionStates.add(build._id!);
    addBuildUpdate(
      app,
      build._id!,
      CREATE_BUILD,
      "Create Build",
      { stdout: "Build Created: " + build.name },
      user.username,
      message.note
    );
    if (build.repo) {
      await cloneRepo(app, user, build);
    }
    return build;
  } catch (err) {
    addSystemUpdate(
      app,
      CREATE_BUILD,
      "Create Build (ERROR)",
      { stderr: JSON.stringify(err) },
      user.username,
      message.note,
      true
    );
  }
}

export default createBuild;
