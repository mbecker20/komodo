import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import fastifyWebsocket from "fastify-websocket";
import { createObservable } from "@monitor/util";
import handleMessage from "../messages";
import { Action } from "@monitor/types";

declare module "fastify" {
  interface FastifyInstance {
    broadcast: <MessageType>(type: string, message: MessageType) => void;
  }
}

const ws = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.register(fastifyWebsocket);

  const messages = createObservable();

  app.decorate(
    "broadcast",
    <MessageType>(type: string, msg: Action & MessageType) => {
      msg.type = type;
      messages.publish(msg);
    }
  );

  app.get("/ws", { websocket: true, onRequest: [app.auth] }, async (connection, req) => {
    const user = await app.users.findById(req.user.id);
    if (user) {
      const unsub = messages.subscribe((msg) => connection.socket.send(msg));
      connection.socket.on("message", (msg) =>
        handleMessage(app, connection.socket, JSON.parse(msg.toString()), user)
      );
      connection.socket.on("close", unsub);
    } else {
      connection.socket.close(403);
    }
  });

  done();
});

export default ws;
