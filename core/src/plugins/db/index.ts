import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import mongoose from "mongoose";
import { MONGO_URL } from "../../config";
import { Build, Deployment, Server, Update, User } from "@monitor/types";
import users from "./users";
import updates from "./updates";
import deployments from "./deployments";
import builds from "./builds";
import servers from "./servers";
import { Model } from "../../util/model";

declare module "fastify" {
  interface FastifyInstance {
    mongoose: typeof mongoose;
    users: Model<User>;
    deployments: Model<Deployment>;
    builds: Model<Build>;
    updates: Model<Update>;
    servers: Model<Server>;
    core: Server;
  }
}

const db = fp(async (app: FastifyInstance, _: {}, done: () => void) => {
	mongoose.connect(MONGO_URL);

	app.decorate("mongoose", mongoose);

  app
    .register(users)
    .register(servers)
    .register(deployments)
    .register(builds)
    .register(updates);

  app.decorate("core", await app.servers.findOne({ isCore: true }));

	done();
});

export default db;