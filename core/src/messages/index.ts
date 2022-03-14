import { Action } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";
import buildMessages from "./build";
import deploymentMessages from "./deployment";
import serverMessages from "./server";

export default async function handleMessage(
  app: FastifyInstance,
  client: WebSocket,
  message: Action & object
) {
  // handle permissions here
  const permissions = 0;
  
  (await buildMessages(app, client, message, permissions)) ||
    (await deploymentMessages(app, client, message, permissions)) ||
    (await serverMessages(app, client, message, permissions));
}
