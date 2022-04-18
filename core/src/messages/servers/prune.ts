import { User } from "@monitor/types";
import {
  PRUNE_IMAGES,
  PRUNE_NETWORKS,
} from "@monitor/util";
import { pruneImages, pruneNetworks } from "@monitor/util-node";
import { FastifyInstance } from "fastify";
import { WebSocket } from "ws";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { sendAlert } from "../../util/helpers";
import { prunePeripheryNetworks } from "../../util/periphery/networks";
import { prunePeripheryImages } from "../../util/periphery/server";
import { addServerUpdate } from "../../util/updates";

export async function pruneServerImages(
  app: FastifyInstance,
  client: WebSocket,
  user: User,
  { serverID, note }: { serverID: string; note?: string }
) {
  if (app.serverActionStates.busy(serverID)) {
    sendAlert(client, "bad", "server busy, try again in a bit");
    return;
  }
  if (user.permissions! < 2) {
    addServerUpdate(
      app,
      serverID,
      PRUNE_IMAGES,
      "Prune Images (DENIED)",
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
    ? await pruneImages()
    : await prunePeripheryImages(server);
  addServerUpdate(
    app,
    serverID,
    PRUNE_IMAGES,
    command,
    log,
    user.username,
    note,
    isError
  );
}

export async function pruneServerNetworks(
  app: FastifyInstance,
  client: WebSocket,
  user: User,
  { serverID, note }: { serverID: string; note?: string }
) {
  if (app.serverActionStates.busy(serverID)) {
    sendAlert(client, "bad", "server busy, try again in a bit");
    return;
  }
  if (user.permissions! < 2) {
    addServerUpdate(
      app,
      serverID,
      PRUNE_NETWORKS,
      "Prune Networks (DENIED)",
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
    ? await pruneNetworks()
    : await prunePeripheryNetworks(server);
  addServerUpdate(
    app,
    serverID,
    PRUNE_NETWORKS,
    command,
    log,
    user.username,
    note,
    isError
  );
}
