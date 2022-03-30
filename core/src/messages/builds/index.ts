import { User } from "@monitor/types";
import {
  ALERT,
  BUILD,
  CLONE_DEPLOYMENT_REPO,
  CREATE_BUILD,
  DELETE_BUILD,
  PULL,
  UPDATE_BUILD,
} from "@monitor/util";
import { FastifyInstance } from "fastify";
import { remove } from "fs-extra";
import { WebSocket } from "ws";
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
      } else {
        sendAlert(client, "bad", "update not successful");
      }
      return true;

    case PULL:
      message.buildID && (await pullRepo(app, user, message));
      return true;

    case CLONE_DEPLOYMENT_REPO:
      if (message.buildID) {
        const build = await app.builds.findById(message.buildID);
        if (!build) {
          sendAlert(client, "bad", "could not find build");
          return true;
        }
        await remove(BUILD_REPO_PATH + build.pullName).catch();
        if (build.repo) {
          await cloneRepo(app, user, build);
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
