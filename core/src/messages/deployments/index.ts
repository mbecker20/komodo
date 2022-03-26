import { User } from "@monitor/types";
import {
  COPY_ENV,
  CREATE_DEPLOYMENT,
  DELETE_CONTAINER,
  DELETE_DEPLOYMENT,
  DEPLOY,
  REFRESH_CONTAINER_STATUS,
  START_CONTAINER,
  STOP_CONTAINER,
  UPDATE_DEPLOYMENT,
  CONTAINER_LOG
} from "@monitor/util";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";
import {
  deleteDeploymentContainer,
  startDeploymentContainer,
  stopDeploymentContainer,
} from "./container";
import createDeployment from "./create";
import deleteDeployment from "./delete";
import deployDeployment from "./deploy";
import updateDeployment from "./update";
import containerLog from "./log";

const DEPLOY_TIMEOUT = 1000;
const DEPLOY_RECHECK_TIMEOUT = 3000;

async function deploymentMessages(
  app: FastifyInstance,
  client: WebSocket,
  message: any,
  user: User
) {
  switch (message.type) {
    case CREATE_DEPLOYMENT:
      const created =
        message.deployment && (await createDeployment(app, user, message));
      if (created) {
        app.broadcast(CREATE_DEPLOYMENT, { deployment: { ...created, status: "not created" } });
      }
      return true;

    case DELETE_DEPLOYMENT:
      const deleted =
        message.deploymentID && (await deleteDeployment(app, user, message));
      if (deleted) {
        app.broadcast(DELETE_DEPLOYMENT, {
          deploymentID: message.deploymentID,
        });
      }
      return true;

    case UPDATE_DEPLOYMENT:
      const updated =
        message.deployment && (await updateDeployment(app, user, message));
      if (updated) {
        app.broadcast(UPDATE_DEPLOYMENT, { deployment: updated });
      }
      return true;

    case DEPLOY:
      await deployDeployment(app, user, message);
      return true;

    case START_CONTAINER:
      await startDeploymentContainer(app, user, message);
      return true;

    case STOP_CONTAINER:
      await stopDeploymentContainer(app, user, message);
      return true;

    case DELETE_CONTAINER:
      await deleteDeploymentContainer(app, user, message);
      return true;

    case CONTAINER_LOG:
      const log = await containerLog(app, message);
      client.send(JSON.stringify(log));
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
