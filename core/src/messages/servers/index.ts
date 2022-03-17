import { Action, User } from "@monitor/types";
import { ADD_SERVER, GET_SERVER_STATS, PRUNE_SERVER, REMOVE_SERVER, UPDATE_SERVER } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";
import addServer from "./add";
import pruneServer from "./prune";
import removeServer from "./remove";
import updateServer from "./update";

async function serverMessages(
  app: FastifyInstance,
  client: WebSocket,
  message: any,
  user: User
) {
  switch (message.type) {
    case ADD_SERVER:
      const created = message.server && (await addServer(app, user, message));
      if (created) {
        app.broadcast(ADD_SERVER, { server: created });
      }
      return true;

    case REMOVE_SERVER:
      const removed =
        message.serverID && (await removeServer(app, user, message));
      if (removed) {
        app.broadcast(REMOVE_SERVER, { serverID: message.serverID });
      }
      return true;

    case UPDATE_SERVER:
      const updated =
        message.server && (await updateServer(app, user, message));
      if (updated) {
        app.broadcast(UPDATE_SERVER, { server: updated });
      }
      return true;

    case PRUNE_SERVER:
      await pruneServer(app, user, message);
      return true;

    case GET_SERVER_STATS:
      return true;

    default:
      return false;
  }
}

export default serverMessages;
