import { User } from "@monitor/types";
import {
  createNetwork,
  CREATE_NETWORK,
  deleteNetwork,
  DELETE_NETWORK,
} from "@monitor/util";
import { FastifyInstance } from "fastify";
import { PERMISSIONS_DENY_LOG } from "../../config";
import {
  createPeripheryNetwork,
  deletePeripheryNetwork,
} from "../../util/periphery/networks";
import { addServerUpdate } from "../../util/updates";

export async function createServerNetwork(
  app: FastifyInstance,
  user: User,
  {
    serverID,
    name,
    driver,
    note,
  }: { serverID: string; name: string; driver?: string; note?: string }
) {
  if (user.permissions! < 2) {
    addServerUpdate(
      app,
      serverID,
      CREATE_NETWORK,
      "Create Network (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  const server = await app.servers.findById(serverID);
  if (!server) return;
  const { command, log, isError } = server.isCore
    ? await createNetwork(name, driver)
    : await createPeripheryNetwork(server, name, driver);
  addServerUpdate(
    app,
    serverID,
    CREATE_NETWORK,
    command,
    log,
    user.username,
    note,
    isError
  );
}

export async function deleteServerNetwork(
  app: FastifyInstance,
  user: User,
  { serverID, name, note }: { serverID: string; name: string; note?: string }
) {
  if (user.permissions! < 2) {
    addServerUpdate(
      app,
      serverID,
      DELETE_NETWORK,
      "Delete Network (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  const server = await app.servers.findById(serverID);
  if (!server) return;
  const { command, log, isError } = server.isCore
    ? await deleteNetwork(name)
    : await deletePeripheryNetwork(server, name);
  addServerUpdate(
    app,
    serverID,
    DELETE_NETWORK,
    command,
    log,
    user.username,
    note,
    isError
  );
}
