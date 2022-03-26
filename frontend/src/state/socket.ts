import { Update } from "@monitor/types";
import { client, pushNotification, WS_URL } from "..";
import {
  ADD_SERVER,
  ADD_UPDATE,
  CREATE_BUILD,
  CREATE_DEPLOYMENT,
  DELETE_BUILD,
  DELETE_DEPLOYMENT,
  REMOVE_SERVER,
  UPDATE_BUILD,
  UPDATE_DEPLOYMENT,
  UPDATE_SERVER,
} from "../state/actions";
import { readableOperation } from "../util/helpers";
import { State } from "./StateProvider";

function socket(state: State) {
  const ws = new WebSocket(WS_URL);

  ws.addEventListener("open", () => {
    ws.send(JSON.stringify({ token: client.token }));
  });

  ws.addEventListener("message", ({ data }) => {
    const message = JSON.parse(data);
    console.log(message);
    handleMessage(state, message);
  });

  ws.addEventListener("close", () => {
    console.log("connection closed");
  });

  return {
    send: <T>(type: string, message: T) => {
      ws.send(JSON.stringify({ ...message, type }));
    },
    close: () => ws.close(),
  };
}

function handleMessage(
  { deployments, builds, servers, updates }: State,
  message: { type: string } & any
) {
  switch (message.type) {
    /* Deployments */
    case CREATE_DEPLOYMENT:
      deployments.add(message.deployment);
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
      break;
  }
}

export default socket;
