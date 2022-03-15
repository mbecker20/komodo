import { Build, User } from "@monitor/types";
import { buildChangelog, clone, dockerBuild, pull } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { move, pathExists, remove } from "fs-extra";
import { WebSocket } from "ws";
import { PERMISSIONS_DENY_LOG, REGISTRY_URL, REPO_PATH } from "../config";
import { BUILDING, PULLING } from "../plugins/actionStates";
import { toDashedName } from "../util/helpers";
import { addBuildUpdate, addSystemUpdate } from "../util/updates";

const CREATE_BUILD = "CREATE_BUILD";
const DELETE_BUILD = "DELETE_BUILD";
const UPDATE_BUILD = "UPDATE_BUILD";
const PULL = "PULL";
const BUILD = "BUILD";
const CLONE_REPO = "CLONE_REPO";

async function buildMessages(
  app: FastifyInstance,
  client: WebSocket,
  message: any,
  user: User
) {
  switch (message.type) {
    case CREATE_BUILD:
      const created = message.build && (await createBuild(app, user, message));
      if (created) {
        app.broadcast(CREATE_BUILD, { build: created });
      }
      return true;

    case DELETE_BUILD:
      const deleted =
        message.buildID && (await deleteBuild(app, user, message));
      if (deleted) {
        app.broadcast(DELETE_BUILD, { buildID: message.buildID });
      }
      return true;

    case UPDATE_BUILD:
      const updated = message.build && (await updateBuild(app, user, message));
      if (updated) {
        app.broadcast(UPDATE_BUILD, { build: updated });
      }
      return true;

    case PULL:
      message.buildID && (await pullRepo(app, user, message));
      return true;

    case BUILD:
      message.buildID && (await build(app, user, message));
      return true;

    default:
      return false;
  }
}

async function createBuild(
  app: FastifyInstance,
  user: User,
  message: { build: Build; note?: string }
) {
  if (user.permissions! >= 1) {
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
  } else {
    addSystemUpdate(
      app,
      CREATE_BUILD,
      "Create Build (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      message.note,
      true
    );
  }
}

async function deleteBuild(
  app: FastifyInstance,
  user: User,
  { buildID, note }: { buildID: string; note?: string }
) {
  const build = await app.builds.findById(buildID).catch(() => {});
  if (!build) return;
  if (user.permissions! >= 2 || user.username === build.owner) {
    try {
      await app.builds.findByIdAndDelete(buildID);
      await app.deployments.updateMany(
        { buildID: build._id },
        { buildID: undefined }
      );
      if (build!.repo) await remove(REPO_PATH + build.imageName);
      app.buildActionStates.delete(buildID);
      addSystemUpdate(
        app,
        DELETE_BUILD,
        "Delete Build",
        {},
        user.username,
        note
      );
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
  } else {
    addSystemUpdate(
      app,
      DELETE_BUILD,
      "Delete Build (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
  }
}

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

async function cloneRepo(
  app: FastifyInstance,
  user: User,
  { imageName, branch, repo, accessToken, _id }: Build
) {
  const { command, log, isError } = await clone(
    repo!,
    imageName!,
    branch,
    accessToken
  );
  addBuildUpdate(
    app,
    _id!,
    CLONE_REPO,
    command,
    log,
    user.username,
    "",
    !isError
  );
}

async function pullRepo(
  app: FastifyInstance,
  user: User,
  { buildID, note }: { buildID: string; note?: string }
) {
  const build = await app.builds.findById(buildID);
  if (!build) return;
  if (user.permissions! >= 2 || build.owner === user.username) {
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
  } else {
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
  }
}

async function build(
  app: FastifyInstance,
  user: User,
  { buildID, note }: { buildID: string; note?: string }
) {
  const build = await app.builds.findById(buildID);
  if (!build) return;
  if (user.permissions! >= 2 || build.owner === user.username) {
    app.buildActionStates.set(buildID, BUILDING, true);
    app.broadcast(BUILD, { complete: false, buildID });
    try {
      const { command, log, isError } = await dockerBuild(
        build,
        REPO_PATH,
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
  } else {
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
  }
}

export default buildMessages;
