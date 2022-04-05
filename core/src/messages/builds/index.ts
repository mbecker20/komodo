import { User } from "@monitor/types";
import {
  BUILD,
  CLONE_BUILD_REPO,
  CREATE_BUILD,
  DELETE_BUILD,
  PULL_BUILD,
  UPDATE_BUILD,
} from "@monitor/util";
import { FastifyInstance } from "fastify";
import { remove } from "fs-extra";
import { WebSocket } from "ws";
import { join } from "path";
import { BUILD_REPO_PATH } from "../../config";
import { sendAlert } from "../../util/helpers";
import build from "./build";
import cloneRepo from "./clone";
import createBuild from "./create";
import deleteBuild from "./delete";
import pullRepo from "./pull";
import updateBuild from "./update";

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
      message.buildID && (await deleteBuild(app, user, message));
      return true;

    case UPDATE_BUILD:
      const updated = message.build && (await updateBuild(app, user, message));
      if (updated) {
        app.broadcast(UPDATE_BUILD, { build: updated });
      } else {
        sendAlert(client, "bad", "update not successful");
      }
      return true;

    case PULL_BUILD:
      message.buildID && (await pullRepo(app, user, message));
      return true;

    case CLONE_BUILD_REPO:
      if (message.buildID) {
        const build = await app.builds.findById(message.buildID);
        if (!build) {
          sendAlert(client, "bad", "could not find build");
          return true;
        }
        await remove(join(BUILD_REPO_PATH, build.pullName!)).catch();
        if (build.repo) {
          app.broadcast(CLONE_BUILD_REPO, {
            buildID: message.buildID,
            complete: false,
          });
          app.buildActionStates.set(message.buildID, "cloning", true);
          await cloneRepo(app, user, build);
          app.buildActionStates.set(message.buildID, "cloning", false);
          app.broadcast(CLONE_BUILD_REPO, {
            buildID: message.buildID,
            complete: true,
          });
        } else {
          sendAlert(client, "bad", "build has no repo configured");
        }
      }
      return true;

    case BUILD:
      message.buildID && (await build(app, user, message));
      return true;

    default:
      return false;
  }
}

export default buildMessages;
