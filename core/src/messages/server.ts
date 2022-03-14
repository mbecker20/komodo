import { FastifyInstance } from "fastify";

const ADD_SERVER = "ADD_SERVER";
const REMOVE_SERVER = "REMOVE_SERVER";
const UPDATE_SERVER = "UPDATE_SERVER";
const PRUNE_SERVER = "PRUNE_SERVER";
const GET_SERVER_STATS = "GET_SERVER_STATS";

async function serverMessages(
  app: FastifyInstance,
  type: string,
  message: any,
  permissions: number
) {
  switch (type) {
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
