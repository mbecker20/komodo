import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import fastifyWebsocket from "fastify-websocket";
import { createObservable } from "@monitor/util";
import handleMessage from "./messages";

declare module "fastify" {
  interface FastifyInstance {
    broadcast: (message: object) => void;
  }
}

const ws = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.register(fastifyWebsocket);

  const messages = createObservable();

  app.decorate("broadcast", (msg: object) => messages.publish(msg));

  app.get("/ws", { websocket: true, onRequest: [app.auth] }, (connection) => {
    const unsub = messages.subscribe((msg) => connection.socket.send(msg));

    connection.socket.on("message", (msg) => handleMessage(app, msg));

    connection.socket.on("close", unsub);
  });

  done();
});

export default ws;
