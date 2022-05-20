import { Server } from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";
import { CPU_USAGE_NOTIFY_LIMIT, DISK_USAGE_NOTIFY_LIMIT, MEM_USAGE_NOTIFY_LIMIT } from "../../config";
import model from "../../util/model";

const servers = fp((app: FastifyInstance, _: {}, done: () => void) => {
  const schema = new Schema<Server>({
    name: { type: String, unique: true },
    address: { type: String, unique: true },
    enabled: { type: Boolean, default: true },
    isCore: Boolean,
    owners: { type: [String], default: [] },
    toNotify: { type: [String], default: [] },
    cpuAlert: { type: Number, default: CPU_USAGE_NOTIFY_LIMIT },
    memAlert: { type: Number, default: MEM_USAGE_NOTIFY_LIMIT },
    diskAlert: { type: Number, default: DISK_USAGE_NOTIFY_LIMIT },
    passkey: String,
    region: String,
    instanceID: String,
  });

  app.decorate("servers", model(app, "Server", schema));

  done();
});

export default servers;
