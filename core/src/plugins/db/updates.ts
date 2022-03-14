import { Update } from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";
import model from "../../util/model";
import { Log } from "./misc";

const updates = fp((app: FastifyInstance, _: {}, done: () => void) => {
	const schema = new Schema<Update>({
    buildID: { type: String, index: true },
    deploymentID: { type: String, index: true },
    serverID: { type: String, index: true },
    operation: { type: String, index: true },
    command: String,
    log: Log,
    timestamp: Number,
    note: String,
    isError: Boolean,
    operator: { type: String, index: true }, // the userID or username
  });
	
	app.decorate("updates", model(app, "Update", schema));
	
	done();
});

export default updates;