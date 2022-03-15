import { Action, User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";

const ADD_SERVER = "ADD_SERVER";
const REMOVE_SERVER = "REMOVE_SERVER";
const UPDATE_SERVER = "UPDATE_SERVER";
const PRUNE_SERVER = "PRUNE_SERVER";
const GET_SERVER_STATS = "GET_SERVER_STATS";

async function serverMessages(
  app: FastifyInstance,
  client: WebSocket,
  message: Action & object,
  user: User
) {
  switch (message.type) {
    case ADD_SERVER:
      return true;

    case REMOVE_SERVER:
      return true;

    case UPDATE_SERVER:
      return true;

    case PRUNE_SERVER:
      return true;

    case GET_SERVER_STATS:
      return true;

    default:
      return false;
  }
}

export default serverMessages;
