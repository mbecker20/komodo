import { User } from "@monitor/types";
import {
  CREATE_DEPLOYMENT,
  DELETE_CONTAINER,
  DELETE_DEPLOYMENT,
  DEPLOY,
  PULL_DEPLOYMENT,
  START_CONTAINER,
  STOP_CONTAINER,
  UPDATE_DEPLOYMENT,
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
import pullDeploymentRepo from "./pull";
import updateDeployment from "./update";

async function deploymentMessages(
  app: FastifyInstance,
  client: WebSocket,
  message: any,
  user: User
) {
  switch (message.type) {
    case CREATE_DEPLOYMENT:
      const created =
        message.deployment &&
        (await createDeployment(app, client, user, message));
      if (created) {
        app.broadcast(CREATE_DEPLOYMENT, {
          deployment: { ...created, status: "not deployed" },
        });
      }
      return true;

    case DELETE_DEPLOYMENT:
      message.deploymentID &&
        (await deleteDeployment(app, client, user, message));
      return true;

    case UPDATE_DEPLOYMENT:
      const updated =
        message.deployment &&
        (await updateDeployment(app, client, user, message));
      if (updated) {
        app.broadcast(UPDATE_DEPLOYMENT, { deployment: updated });
      }
      return true;

    case DEPLOY:
      await deployDeployment(app, client, user, message);
      return true;

    case START_CONTAINER:
      await startDeploymentContainer(app, client, user, message);
      return true;

    case STOP_CONTAINER:
      await stopDeploymentContainer(app, client, user, message);
      return true;

    case DELETE_CONTAINER:
      await deleteDeploymentContainer(app, client, user, message);
      return true;

    case PULL_DEPLOYMENT:
      await pullDeploymentRepo(app, client, user, message);
      return true;

    default:
      return false;
  }
}

export default deploymentMessages;
