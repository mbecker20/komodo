import { Update, User } from "@monitor/types";
import { client, pushNotification, WS_URL } from "..";
import {
  ADD_SERVER,
  ADD_UPDATE,
  CREATE_BUILD,
  CREATE_DEPLOYMENT,
  DELETE_BUILD,
  DELETE_CONTAINER,
  DELETE_DEPLOYMENT,
  DEPLOY,
  REMOVE_SERVER,
  START_CONTAINER,
  STOP_CONTAINER,
  UPDATE_BUILD,
  UPDATE_DEPLOYMENT,
  UPDATE_SERVER,
} from "../state/actions";
import { readableOperation } from "../util/helpers";
import { getDeploymentStatus, getServer } from "../util/query";
import { useSelected } from "./hooks";
import { State } from "./StateProvider";

function socket(
  user: User,
  state: State,
  selected: ReturnType<typeof useSelected>
) {
  const ws = new WebSocket(WS_URL);

  ws.addEventListener("open", () => {
    ws.send(JSON.stringify({ token: client.token }));
  });

  ws.addEventListener("message", ({ data }) => {
    const message = JSON.parse(data);
    console.log(message);
    handleMessage(user, state, selected, message);
  });

  ws.addEventListener("close", () => {
    console.log("connection closed");
  });

  return {
    subscribe: (
      types: string[],
      callback: (message: { type: string } & any) => void
    ) => {
      const listener = ({ data }: { data: string }) => {
        const message = JSON.parse(data);
        if (types.includes(message.type)) {
          callback(JSON.parse(data));
        }
      };
      ws.addEventListener("message", listener);
      return () => {
        ws.removeEventListener("message", listener);
      };
    },
    send: <T>(type: string, message: T) => {
      ws.send(JSON.stringify({ ...message, type }));
    },
    close: () => ws.close(),
  };
}

function handleMessage(
  user: User,
  { deployments, builds, servers, updates }: State,
  selected: ReturnType<typeof useSelected>,
  message: { type: string } & any
) {
  switch (message.type) {
    /* Deployments */
    case CREATE_DEPLOYMENT:
      deployments.add(message.deployment);
      if (message.deployment.owners[0] === user.username) {
        selected.set(message.deployment._id, "deployment");
      }
      break;

    case DELETE_DEPLOYMENT:
      deployments.delete(message.deploymentID);
      break;

    case UPDATE_DEPLOYMENT:
      deployments.update(message.deployment);
      break;

    /* Builds */
    case CREATE_BUILD:
      builds.add(message.build);
      break;

    case DELETE_BUILD:
      builds.delete(message.buildID);
      break;

    case UPDATE_BUILD:
      builds.update(message.build);
      break;

    /* Servers */
    case ADD_SERVER:
      servers.add(message.server);
      break;

    case REMOVE_SERVER:
      servers.delete(message.serverID);
      break;

    case UPDATE_SERVER:
      servers.update(message.server);
      break;

    /* Updates */
    case ADD_UPDATE:
      const { update } = message as { update: Update };
      updates.add(update);
      pushNotification(
        update.isError ? "bad" : "good",
        `${readableOperation(update.operation)} by ${update.operator}`
      );
      if (update.deploymentID === selected.id()) {
        if (
          [DEPLOY, START_CONTAINER, STOP_CONTAINER, DELETE_CONTAINER].includes(
            update.operation
          )
        ) {
          getDeploymentStatus(selected.id()).then((status) =>
            deployments.update({ ...deployments.get(selected.id())!, status })
          );
        }
      } else if (update.serverID === selected.id()) {
        if ([UPDATE_SERVER].includes(update.operation)) {
          getServer(selected.id()).then((server) => {
            servers.update(server);
          });
        }
      }
      break;
  }
}

export default socket;
