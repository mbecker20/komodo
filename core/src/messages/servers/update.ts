import { Server, User } from "@monitor/types";
import { serverChangelog } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { UPDATE_SERVER } from ".";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { addServerUpdate } from "../../util/updates";

async function updateServer(
  app: FastifyInstance,
  user: User,
  { server }: { server: Server }
) {
  if (user.permissions! < 2) {
    addServerUpdate(
      app,
      server._id!,
      UPDATE_SERVER,
      "Update Sever (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      "",
      true
    );
    return;
  }
  const preServer = await app.servers.findById(server._id!);
  if (!preServer) return;
  await app.servers.updateById(server._id!, server);
  addServerUpdate(
    app,
    server._id!,
    UPDATE_SERVER,
    "Update Sever",
    {
      stdout: serverChangelog(preServer, server),
    },
    user.username
  );
  return server;
}

export default updateServer;
