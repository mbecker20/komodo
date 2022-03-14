import { Action } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";

const CREATE_BUILD = "CREATE_BUILD";
const DELETE_BUILD = "DELETE_BUILD";
const UPDATE_BUILD = "UPDATE_BUILD";
const PULL = "PULL";
const BUILD = "BUILD";

async function buildMessages(app: FastifyInstance, client: WebSocket, message: Action & object, permissions: number) {
	switch (message.type) {
    case CREATE_BUILD:
      return true;

    case DELETE_BUILD:
      return true;

    case UPDATE_BUILD:
      return true;

    case PULL:
      return true;

    case BUILD:
      return true;

    default:
      return false;
  }
}

export default buildMessages;
