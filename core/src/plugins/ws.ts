import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import fastifyWebsocket from "fastify-websocket";
import { createDecoder } from "fast-jwt";
import { createUserObservable } from "@monitor/util";
import handleMessage from "../messages";
import { Action, Build, Deployment, Server } from "@monitor/types";

declare module "fastify" {
  interface FastifyInstance {
    broadcast: <MessageType>(
      type: string,
      message: MessageType,
      userFilter: (userID: string) => Promise<boolean>
    ) => void;
    deploymentUserFilter: (
      deploymentID: string,
      deleted?: Deployment
    ) => (userID: string) => Promise<boolean>;
    serverUserFilter: (
      serverID: string,
      deleted?: Server
    ) => (userID: string) => Promise<boolean>;
    buildUserFilter: (
      buildID: string,
      deleted?: Build
    ) => (userID: string) => Promise<boolean>;
    adminUserFilter: (userID: string) => Promise<boolean>;
  }
}

const decode = createDecoder();

const ws = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.register(fastifyWebsocket);

  const messages = createUserObservable<string>();

  app.decorate(
    "broadcast", // used to sending state updates to all connected clients
    <MessageType>(
      type: string,
      msg: Action & MessageType,
      userFilter: (userID: string) => Promise<boolean>
    ) => {
      msg.type = type;
      messages.publish(msg, userFilter);
    }
  );

  app.decorate(
    "deploymentUserFilter",
    (deploymentID: string, deleted?: Deployment) => async (userID: string) => {
      const user = await app.users.findById(userID);
      if (!user || !user.enabled) return false;
      if (user.permissions! > 1) return true;
      const deployment =
        deleted || (await app.deployments.findById(deploymentID, "owners"));
      if (deployment?.owners.includes(user.username)) return true;
      return false;
    }
  );

  app.decorate(
    "serverUserFilter",
    (serverID: string, deleted?: Server) => async (userID: string) => {
      const user = await app.users.findById(userID);
      if (!user || !user.enabled) return false;
      if (user.permissions! > 1) return true;
      const server =
        deleted || (await app.servers.findById(serverID, "owners"));
      if (server?.owners.includes(user.username)) return true;
      return false;
    }
  );

  app.decorate(
    "buildUserFilter",
    (buildID: string, deleted?: Build) => async (userID: string) => {
      const user = await app.users.findById(userID);
      if (!user || !user.enabled) return false;
      if (user.permissions! > 1) return true;
      const build = deleted || (await app.builds.findById(buildID, "owners"));
      if (build?.owners.includes(user.username)) return true;
      return false;
    }
  );

  app.decorate("adminUserFilter", async (userID: string) => {
    const user = await app.users.findById(userID);
    if (!user || !user.enabled) return false;
    if (user.permissions! > 1) return true;
    return false;
  });

  app.get("/ws", { websocket: true }, async (connection) => {
    connection.socket.on("message", async (msg) => {
      try {
        const jwt = JSON.parse(msg.toString()).token;
        if (jwt && app.jwt.verify(jwt)) {
          const payload = decode(jwt) as { id: string };
          const userID = payload.id;
          const user = await app.users.findById(userID);
          if (user && user.enabled) {
            const unsub = messages.subscribe(userID, (msg) =>
              connection.socket.send(msg)
            );
            connection.socket.removeAllListeners("message");
            connection.socket.on("message", (raw) => {
              const msg = raw.toString();
              if (msg === "PING") {
                connection.socket.send("PONG");
                return;
              }
              handleMessage(app, connection.socket, JSON.parse(msg), userID);
            });
            connection.socket.on("close", unsub);
            connection.socket.send(
              JSON.stringify({
                type: "LOGIN",
                message: "logged in successfully",
              })
            );
          } else {
            connection.socket.close();
          }
        } else {
          connection.socket.close();
        }
      } catch (error) {
        console.log("WEBSOCKET ERROR");
        console.log(error);
      }
    });
  });

  done();
});

export default ws;
