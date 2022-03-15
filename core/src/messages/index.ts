import { Action, User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";
import buildMessages from "./builds";
import deploymentMessages from "./deployments";
import serverMessages from "./server";

export default async function handleMessage(
  app: FastifyInstance,
  client: WebSocket,
  message: Action & object,
  user: User
) {
  (await buildMessages(app, client, message, user)) ||
    (await deploymentMessages(app, client, message, user)) ||
    (await serverMessages(app, client, message, user));
}
