import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import mongoose, { Model } from "mongoose";
import { MONGO_URL } from "../../config";
import { Build, Deployment, Update, User } from "@monitor/types";
import users from "./users";
import updates from "./updates";
import deployments from "./deployments";
import builds from "./builds";

declare module "fastify" {
  interface FastifyInstance {
    mongoose: typeof mongoose;
    users: Model<User>;
    deployments: Model<Deployment>;
    builds: Model<Build>;
    updates: Model<Update>;
  }
}

const db = fp((app: FastifyInstance, _: {}, done: () => void) => {
	mongoose.connect(MONGO_URL);

	app.decorate("mongoose", mongoose);

  app
    .register(users)
    .register(deployments)
    .register(builds)
    .register(updates);

	done();
});

export default db;