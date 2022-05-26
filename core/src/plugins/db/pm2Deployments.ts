import { Pm2Deployment } from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";
import { Command } from "./misc";

const pm2Deployments = fp((app: FastifyInstance, _: {}, done: () => void) => {
	const schema = new Schema<Pm2Deployment>({
		name: { type: String, unique: true, index: true },
		pullName: { type: String, unique: true, index: true },
		serverID: { type: String, index: true },
		owners: { type: [String], default: [] },
		repo: String,
		branch: String,
		subfolder: String,
		githubAccount: String,
		onPull: Command,
		onClone: Command,
	});
	
	app.decorate("pm2Deployments", app.mongoose.model("Pm2Deployment", schema));
	
	done();
});

export default pm2Deployments;