import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import mongoose, { Model } from "mongoose";
import { MONGO_URL } from "../config";
import { User } from "@monitor/types";

declare module "fastify" {
  interface FastifyInstance {
    mongoose: typeof mongoose;
    users: Model<User>;
  }
}

const db = fp((app: FastifyInstance, _: {}, done: () => void) => {
	mongoose.connect(MONGO_URL);

	app.decorate("mongoose", mongoose);

	done();
});

export default db;