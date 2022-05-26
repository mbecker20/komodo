import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import mongoose from "mongoose";
import { MONGO_URL } from "../../config";
import { AccountAccess, Build, Deployment, Pm2Deployment, Server, Update, User } from "@monitor/types";
import users from "./users";
import updates from "./updates";
import deployments from "./deployments";
import builds from "./builds";
import servers from "./servers";
import { Model } from "../../util/model";
import accounts from "./accounts";
import pm2Deployments from "./pm2Deployments";

declare module "fastify" {
  interface FastifyInstance {
    mongoose: typeof mongoose;
    users: Model<User>;
    deployments: Model<Deployment>;
    pm2Deployments: Model<Pm2Deployment>;
    builds: Model<Build>;
    updates: Model<Update>;
    servers: Model<Server>;
    accounts: Model<AccountAccess>;
    core: Server & { _id: string };
  }
}

const db = fp(async (app: FastifyInstance, _: {}, done: () => void) => {
	await mongoose.connect(MONGO_URL);

	app.decorate("mongoose", mongoose);

  app
    .register(users)
    .register(servers)
    .register(deployments)
    .register(pm2Deployments)
    .register(builds)
    .register(updates)
    .register(accounts);

  app.after(async () => {
    const server = await app.servers.findOne({ isCore: true });
    server!._id = server?._id?.toString();
    app.decorate("core", server);
  });

	done();
});

export default db;