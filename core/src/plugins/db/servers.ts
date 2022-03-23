import { Server } from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";
import model from "../../util/model";

const servers = fp((app: FastifyInstance, _: {}, done: () => void) => {
  const schema = new Schema<Server>({
    name: { type: String, unique: true },
    address: String,
    passkey: String,
    enabled: { type: Boolean, default: true },
    isCore: Boolean,
  });

  app.decorate("servers", model(app, "Server", schema));

  done();
});

export default servers;
