import { Action } from "@monitor/types";
import { FastifyInstance } from "fastify";
import buildMessages from "./build";
import deploymentMessages from "./deployment";
import serverMessages from "./server";

export default async function handleMessage(
  app: FastifyInstance,
  message: Action & object
) {
  // handle permissions here
  const permissions = 0;
  (await buildMessages(app, message.type, message, permissions)) ||
    (await deploymentMessages(app, message.type, message, permissions)) ||
    (await serverMessages(app, message.type, message, permissions));
}
