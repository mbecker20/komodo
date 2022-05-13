import { Server, User } from "@monitor/types";
import { serverChangelog, UPDATE_SERVER } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { addServerUpdate } from "../../util/updates";

async function updateServer(
  app: FastifyInstance,
  user: User,
  { server }: { server: Server }
) {
  const preServer = await app.servers.findById(server._id!);
  if (!preServer) return;
  if (user.permissions! < 2 && !preServer.owners.includes(user.username)) {
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
  (server.owners as any) = undefined;
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
