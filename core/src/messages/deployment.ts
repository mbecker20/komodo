import { Action, User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";

const CREATE_DEPLOYMENT = "CREATE_DEPLOYMENT";
const DELETE_DEPLOYMENT = "DELETE_DEPLOYMENT";
const UPDATE_DEPLOYMENT = "UPDATE_DEPLOYMENT";
const DEPLOY = "DEPLOY";
const REDEPLOY = "REDEPLOY";
const START_CONTAINER = "START_CONTAINER";
const STOP_CONTAINER = "STOP_CONTAINER";
const DELETE_CONTAINER = "DELETE_CONTAINER";
const REFRESH_CONTAINER_STATUS = "REFRESH_CONTAINER_STATUS";
const COPY_ENV = "COPY_ENV";

const DEPLOY_TIMEOUT = 1000;
const DEPLOY_RECHECK_TIMEOUT = 3000;

async function deploymentMessages(
  app: FastifyInstance,
  client: WebSocket,
  message: Action & object,
  user: User
) {
	switch (message.type) {
    case CREATE_DEPLOYMENT:
      return true;

    case DELETE_DEPLOYMENT:
      return true;

    case UPDATE_DEPLOYMENT:
      return true;

    case DEPLOY:
      return true;

    case REDEPLOY:
      return true;

    case START_CONTAINER:
      return true;

    case STOP_CONTAINER:
      return true;

    case UPDATE_DEPLOYMENT:
      return true;

    case DELETE_CONTAINER:
      return true;

    case REFRESH_CONTAINER_STATUS:
      return true;

		case COPY_ENV:
			return true;

    default:
      return false;
  }
}

export default deploymentMessages;