import { User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";
import build from "./build";
import createBuild from "./create";
import deleteBuild from "./delete";
import pullRepo from "./pull";
import updateBuild from "./update";

export const CREATE_BUILD = "CREATE_BUILD";
export const DELETE_BUILD = "DELETE_BUILD";
export const UPDATE_BUILD = "UPDATE_BUILD";
export const PULL = "PULL";
export const BUILD = "BUILD";
export const CLONE_REPO = "CLONE_REPO";

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

export default buildMessages;
