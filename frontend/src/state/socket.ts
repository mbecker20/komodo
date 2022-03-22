import { client, WS_URL } from "..";
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
} from "@monitor/util";
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
  };
}

function handleMessage(
  { deployments, builds, servers, updates }: State,
  message: { type: string } & any
) {
  switch (message.type) {
    /* Deployments */
    case CREATE_DEPLOYMENT:
      break;

    case DELETE_DEPLOYMENT:
      break;

    case UPDATE_DEPLOYMENT:
      break;

    /* Builds */
    case CREATE_BUILD:
      break;

    case DELETE_BUILD:
      break;

    case UPDATE_BUILD:
      break;

    /* Servers */
    case ADD_SERVER:
      break;

    case REMOVE_SERVER:
      break;

    case UPDATE_SERVER:
      break;

		/* Updates */
		case ADD_UPDATE:
			break;
  }
}

export default socket;
