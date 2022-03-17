import { Action, User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";
import { deleteDeploymentContainer, startDeploymentContainer, stopDeploymentContainer } from "./container";
import createDeployment from "./create";
import deleteDeployment from "./delete";
import deployDeployment from "./deploy";
import updateDeployment from "./update";

export const CREATE_DEPLOYMENT = "CREATE_DEPLOYMENT";
export const DELETE_DEPLOYMENT = "DELETE_DEPLOYMENT";
export const UPDATE_DEPLOYMENT = "UPDATE_DEPLOYMENT";
export const DEPLOY = "DEPLOY";
export const START_CONTAINER = "START_CONTAINER";
export const STOP_CONTAINER = "STOP_CONTAINER";
export const DELETE_CONTAINER = "DELETE_CONTAINER";
export const REFRESH_CONTAINER_STATUS = "REFRESH_CONTAINER_STATUS";
export const COPY_ENV = "COPY_ENV";
export const CLONE_REPO = "CLONE_REPO";

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
        app.broadcast(CREATE_DEPLOYMENT, { deployment: created });
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
        message.deploymentID && (await updateDeployment(app, user, message));
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

    case REFRESH_CONTAINER_STATUS:
      return true;

    case COPY_ENV:
      return true;

    default:
      return false;
  }
}

export default deploymentMessages;
