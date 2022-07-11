import { Update, User } from "@monitor/types";
import { client, pushNotification, WS_URL } from "..";
import {
  ADD_SERVER,
  ADD_UPDATE,
  ALERT,
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
} from "@monitor/util";
import { readableOperation } from "../util/helpers";
import { getDeploymentStatus, getServer } from "../util/query";
import { State } from "./StateProvider";
import { createSignal } from "solid-js";
import ReconnectingWebSocket from "reconnecting-websocket";

function socket(user: User, state: State) {
  const ws = new ReconnectingWebSocket(WS_URL);

  const [isOpen, setOpen] = createSignal(false);

  ws.addEventListener("open", () => {
    console.log("connection opened");
    ws.send(JSON.stringify({ token: client.token }));
    setOpen(true);
  });

  ws.addEventListener("message", ({ data }) => {
    if (data === "PONG") {
      console.log("pong");
      return;
    }
    const message = JSON.parse(data);
    console.log(message);
    handleMessage(user, state, message);
  });

  const int = setInterval(() => {
    if (ws.readyState === ws.OPEN) {
      ws.send("PING");
    } else {
      setOpen(false);
    }
  }, 10000);

  ws.addEventListener("close", () => {
    console.log("connection closed");
    clearInterval(int);
    setOpen(false);
  });

  return {
    subscribe: (
      types: string[],
      callback: (message: { type: string } & any) => void
    ) => {
      const listener = ({ data }: { data: string }) => {
        if (data === "PONG") {
          return;
        }
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
    isOpen,
  };
}

function handleMessage(
  user: User,
  { deployments, builds, servers, updates, selected }: State,
  message: { type: string } & any
) {
  switch (message.type) {
    /* Deployments */
    case CREATE_DEPLOYMENT:
      deployments.add(message.deployment);
      if (message.deployment.owners[0] === user.username) {
        // selected.set(message.deployment._id, "deployment");
      }
      break;

    case DELETE_DEPLOYMENT:
      if (message.complete) {
        deployments.delete(message.deploymentID);
      }
      break;

    case UPDATE_DEPLOYMENT:
      if (message.deployment) {
        deployments.update(message.deployment);
      }
      break;

    /* Builds */
    case CREATE_BUILD:
      builds.add(message.build);
      if (message.build.owners[0] === user.username) {
        // selected.set(message.build._id, "build");
      }
      break;

    case DELETE_BUILD:
      if (message.complete) {
        builds.delete(message.buildID);
      }
      break;

    case UPDATE_BUILD:
      if (message.build) {
        builds.update(message.build);
      }
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

    case ALERT:
      pushNotification(message.status, message.message);
      break;

    /* Updates */
    case ADD_UPDATE:
      const { update } = message as { update: Update };
      if (
        (update.deploymentID &&
          !deployments
            .get(update.deploymentID)
            ?.owners.includes(user.username)) ||
        (update.buildID &&
          !builds.get(update.buildID)?.owners.includes(user.username)) ||
        (update.serverID &&
          !servers.get(update.serverID)?.owners.includes(user.username))
      ) {
        // dont respond to updates outside of users scope. not airtight for protecting sensitive data.
        return;
      }
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
