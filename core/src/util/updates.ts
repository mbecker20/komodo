import { Log } from "@monitor/types";
import { ADD_UPDATE, timestamp } from "@monitor/util";
import { FastifyInstance } from "fastify";

export async function addBuildUpdate(
  app: FastifyInstance,
  buildID: string,
  operation: string,
  command: string,
  log: Log,
  operator: string,
  note = "",
  isError = false
) {
  const update = await app.updates.create({
    buildID,
    operation,
    command,
    log,
    operator,
    note,
    isError,
    timestamp: timestamp(),
  });
  app.broadcast(ADD_UPDATE, { update });
}

export async function addDeploymentUpdate(
  app: FastifyInstance,
  deploymentID: string,
  operation: string,
  command: string,
  log: Log,
  operator: string,
  note = "",
  isError = false
) {
  const update = await app.updates.create({
    deploymentID,
    operation,
    command,
    log,
    operator,
    note,
    isError,
    timestamp: timestamp(),
  });
  app.broadcast(ADD_UPDATE, { update });
}

export async function addServerUpdate(
  app: FastifyInstance,
  serverID: string,
  operation: string,
  command: string,
  log: Log,
  operator: string,
  note = "",
  isError = false
) {
  const update = await app.updates.create({
    serverID,
    operation,
    command,
    log,
    operator,
    note,
    isError,
    timestamp: timestamp(),
  });
  app.broadcast(ADD_UPDATE, { update });
}

export async function addSystemUpdate(
  app: FastifyInstance,
  operation: string,
  command: string,
  log: Log,
  operator: string,
  note = "",
  isError = false
) {
  const update = await app.updates.create({
    operation,
    command,
    log,
    operator,
    note,
    isError,
    timestamp: timestamp(),
  });
  app.broadcast(ADD_UPDATE, { update });
}