import { User } from "@monitor/types";
import { BUILD, CREATE_BUILD, DELETE_BUILD, PULL, UPDATE_BUILD } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";
import build from "./build";
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
