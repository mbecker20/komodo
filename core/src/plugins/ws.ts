import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import fastifyWebsocket from "fastify-websocket";
import { createDecoder } from "fast-jwt";
import { createObservable } from "@monitor/util";
import handleMessage from "../messages";
import { Action } from "@monitor/types";

declare module "fastify" {
  interface FastifyInstance {
    broadcast: <MessageType>(type: string, message: MessageType) => void;
  }
}

const decode = createDecoder();

const ws = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.register(fastifyWebsocket);

  const messages = createObservable();

  app.decorate(
    "broadcast", // used to sending state updates to all connected clients
    <MessageType>(type: string, msg: Action & MessageType) => {
      msg.type = type;
      messages.publish(msg);
    }
  );

  app.get("/ws", { websocket: true }, async (connection) => {
    connection.socket.on("message", async (msg) => {
      const jwt = JSON.parse(msg.toString()).token;
      if (jwt && app.jwt.verify(jwt)) {
        const payload = decode(jwt) as { id: string };
        const userID = payload.id;
        const user = await app.users.findById(userID);
        if (user && user.enabled) {
          const unsub = messages.subscribe((msg) =>
            connection.socket.send(msg)
          );
          connection.socket.removeAllListeners("message");
          connection.socket.on("message", (raw) => {
            const msg = raw.toString();
            if (msg === "PING") {
              connection.socket.send("PONG");
              return;
            }
            handleMessage(
              app,
              connection.socket,
              JSON.parse(msg),
              userID
            );
          });
          connection.socket.on("close", unsub);
          connection.socket.send(
            JSON.stringify({ type: "LOGIN", message: "logged in successfully" })
          );
        } else {
          connection.socket.close();
        }
      } else {
        connection.socket.close();
      }
    });
  });

  done();
});

export default ws;
