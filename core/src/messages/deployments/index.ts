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
  RECLONE_DEPLOYMENT_REPO,
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
import recloneDeployment from "./reclone";
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
        app.broadcast(
          CREATE_DEPLOYMENT,
          {
            deployment: { ...created, status: "not deployed" },
          },
          app.deploymentUserFilter(created._id!)
        );
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
        app.broadcast(
          UPDATE_DEPLOYMENT,
          { deployment: updated },
          app.deploymentUserFilter(updated._id!)
        );
      }
      return true;

    case DEPLOY:
      message.deploymentID &&
        (await deployDeployment(app, client, user, message));
      return true;

    case START_CONTAINER:
      message.deploymentID &&
        (await startDeploymentContainer(app, client, user, message));
      return true;

    case STOP_CONTAINER:
      message.deploymentID &&
        (await stopDeploymentContainer(app, client, user, message));
      return true;

    case DELETE_CONTAINER:
      message.deploymentID &&
        (await deleteDeploymentContainer(app, client, user, message));
      return true;

    case PULL_DEPLOYMENT:
      message.deploymentID &&
        (await pullDeploymentRepo(app, client, user, message));
      return true;

    case RECLONE_DEPLOYMENT_REPO:
      message.deploymentID &&
        (await recloneDeployment(app, client, user, message));
      return true;

    default:
      return false;
  }
}

export default deploymentMessages;
